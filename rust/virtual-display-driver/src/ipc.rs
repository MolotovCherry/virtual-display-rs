use std::{
    mem::size_of,
    ptr::{addr_of_mut, NonNull},
    sync::{Mutex, OnceLock},
    thread,
};

use driver_ipc::{
    Dimen, DriverCommand, EventCommand, Mode, Monitor, RefreshRate, ReplyCommand, RequestCommand,
    ServerCommand,
};
use log::{error, warn};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt as _},
    net::windows::named_pipe::{NamedPipeServer, ServerOptions},
    sync::broadcast::{self, error::RecvError, Sender},
    task,
};
use wdf_umdf::IddCxMonitorDeparture;
use wdf_umdf_sys::{IDDCX_ADAPTER__, IDDCX_MONITOR__};
use windows::Win32::{
    Security::{
        InitializeSecurityDescriptor, SetSecurityDescriptorDacl, PSECURITY_DESCRIPTOR,
        SECURITY_ATTRIBUTES, SECURITY_DESCRIPTOR,
    },
    System::SystemServices::SECURITY_DESCRIPTOR_REVISION1,
};

use crate::{context::DeviceContext, helpers::LazyLock};

pub static ADAPTER: OnceLock<AdapterObject> = OnceLock::new();
pub static MONITOR_MODES: LazyLock<Mutex<Vec<MonitorObject>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

#[derive(Debug)]
pub struct AdapterObject(pub NonNull<IDDCX_ADAPTER__>);
unsafe impl Sync for AdapterObject {}
unsafe impl Send for AdapterObject {}

#[derive(Debug)]
pub struct MonitorObject {
    pub object: Option<NonNull<IDDCX_MONITOR__>>,
    pub data: Monitor,
}
unsafe impl Sync for MonitorObject {}
unsafe impl Send for MonitorObject {}

const BUFFER_SIZE: u32 = 4096;
// EOT
const EOF: char = '\x04';

// message processor
async fn process_message(
    id: usize,
    server: &mut NamedPipeServer,
    tx: &Sender<(usize, Vec<Monitor>)>,
    buf: &[u8],
    iter: impl Iterator<Item = usize>,
) -> Result<(), ()> {
    // process each message in the buffer
    let mut start = 0;
    for eidx in iter {
        let sidx = start;
        start = eidx + 1;

        let Ok(msg) = std::str::from_utf8(&buf[sidx..eidx]) else {
            continue;
        };

        let Ok(command) = serde_json::from_str::<ServerCommand>(msg) else {
            continue;
        };

        match command {
            // driver commands
            ServerCommand::Driver(cmd) => match cmd {
                DriverCommand::Notify(monitors) => {
                    notify(monitors.clone());
                    _ = tx.send((id, monitors));
                }

                DriverCommand::Remove(ids) => {
                    remove(&ids);

                    let lock = MONITOR_MODES.lock().unwrap();
                    let monitors = lock.iter().map(|m| m.data.clone()).collect();
                    _ = tx.send((id, monitors));
                }

                DriverCommand::RemoveAll => {
                    remove_all();
                    _ = tx.send((id, Vec::new()));
                }

                _ => (),
            },

            // request commands
            ServerCommand::Request(RequestCommand::State) => {
                let mut data = {
                    let lock = MONITOR_MODES.lock().unwrap();
                    let monitors = lock.iter().map(|m| m.data.clone()).collect();
                    let command = ReplyCommand::State(monitors);

                    let Ok(serialized) = serde_json::to_string(&command) else {
                        error!("Command::Request - failed to serialize reply");
                        break;
                    };

                    serialized
                };

                data.push(EOF);

                if server.write_all(data.as_bytes()).await.is_err() {
                    // a server error means we should completely stop trying
                    return Err(());
                }
            }

            // Everything else is an invalid command
            _ => (),
        }
    }

    Ok(())
}

#[allow(clippy::too_many_lines)]
pub fn startup() {
    thread::spawn(move || {
        // These security attributes will allow anyone access, so local account does not need admin privileges to use it

        let mut sd = SECURITY_DESCRIPTOR::default();

        unsafe {
            InitializeSecurityDescriptor(
                PSECURITY_DESCRIPTOR(addr_of_mut!(sd).cast()),
                SECURITY_DESCRIPTOR_REVISION1,
            )
            .unwrap();
        }

        unsafe {
            SetSecurityDescriptorDacl(
                PSECURITY_DESCRIPTOR(addr_of_mut!(sd).cast()),
                true,
                None,
                false,
            )
            .unwrap();
        }

        let mut sa = SECURITY_ATTRIBUTES {
            #[allow(clippy::cast_possible_truncation)]
            nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: addr_of_mut!(sd).cast(),
            bInheritHandle: false.into(),
        };

        // async time!
        let pipe_server = async {
            let (tx, _rx) = broadcast::channel(1);

            let mut id = 0usize;

            loop {
                let mut server = unsafe {
                    ServerOptions::new()
                        .access_inbound(true)
                        .access_outbound(true)
                        .reject_remote_clients(true)
                        .in_buffer_size(BUFFER_SIZE)
                        .out_buffer_size(BUFFER_SIZE)
                        // default is unlimited instances
                        .create_with_security_attributes_raw(
                            r"\\.\pipe\virtualdisplaydriver",
                            std::ptr::from_mut::<SECURITY_ATTRIBUTES>(&mut sa).cast(),
                        )
                        .unwrap()
                };

                if server.connect().await.is_err() {
                    continue;
                }

                id += 1;

                let mut msg_buf: Vec<u8> = Vec::with_capacity(BUFFER_SIZE as usize);
                let mut buf = vec![0; BUFFER_SIZE as usize];
                let tx = tx.clone();
                let mut rx = tx.subscribe();

                task::spawn(async move {
                    loop {
                        tokio::select! {
                            val = server.read(&mut buf) =>  {
                                match val {
                                    // 0 = no more data to read
                                    // or break on err
                                    Ok(0) | Err(_) => break,

                                    Ok(size) => msg_buf.extend(&buf[..size]),
                                }

                                // get all eof boundary positions
                                let eof_iter = msg_buf.iter().enumerate().filter_map(|(i, &byte)| {
                                    if byte == EOF as u8 {
                                        Some(i)
                                    } else {
                                        None
                                    }
                                });

                                if process_message(id, &mut server, &tx, &msg_buf, eof_iter.clone()).await.is_err() {
                                    break;
                                }

                                // remove processed messages from buffer
                                // we can exploit the fact that these are sequential
                                // so just get the last index and chop off everything before that
                                if let Some(last) = eof_iter.last() {
                                    // remove everything up to and including the last EOF
                                    msg_buf.drain(..=last);
                                }
                            },

                            val = rx.recv() => {
                                let data = match val {
                                    // ignore if this value was sent for the current client (current client doesn't need notification)
                                    Ok((client_id, _)) if client_id == id => continue,

                                    Ok((_, data)) => data,

                                    Err(RecvError::Lagged(_)) => continue,

                                    // closed
                                    Err(_) => break
                                };

                                let command = EventCommand::Changed(data);

                                let Ok(mut serialized) = serde_json::to_string(&command) else {
                                    error!("Command::Request - failed to serialize reply");
                                    break;
                                };

                                serialized.push(EOF);

                                if server.write_all(serialized.as_bytes()).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                });
            }
        };

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(pipe_server);
    });
}

/// used to check the validity of a Vec<Monitor>
/// the validity invariants are:
/// 1. unique monitor ids
/// 2. unique monitor modes (width+height must be unique per array element)
/// 3. unique refresh rates per monitor mode
fn has_duplicates(monitors: &[Monitor]) -> bool {
    let mut monitor_iter = monitors.iter();
    while let Some(monitor) = monitor_iter.next() {
        let duplicate_id = monitor_iter.clone().any(|b| monitor.id == b.id);
        if duplicate_id {
            warn!("Found duplicate monitor id {}", monitor.id);
            return true;
        }

        let mut mode_iter = monitor.modes.iter();
        while let Some(mode) = mode_iter.next() {
            let duplicate_mode = mode_iter
                .clone()
                .any(|m| mode.height == m.height && mode.width == m.width);
            if duplicate_mode {
                warn!(
                    "Found duplicate mode {}x{} on monitor {}",
                    mode.width, mode.height, monitor.id
                );
                return true;
            }

            let mut refresh_iter = mode.refresh_rates.iter().copied();
            while let Some(rr) = refresh_iter.next() {
                let duplicate_rr = refresh_iter.clone().any(|r| rr == r);
                if duplicate_rr {
                    warn!(
                        "Found duplicate refresh rate {rr} on mode {}x{} for monitor {}",
                        mode.width, mode.height, monitor.id
                    );
                    return true;
                }
            }
        }
    }

    false
}

/// Notifies driver of new system monitor state
///
/// Adds, updates, or removes monitors as needed
///
/// Note that updated monitors causes a detach, update, and reattach. (Required for windows to see the changes)
///
/// Only detaches/reattaches if required
/// e.g. only a monitor name update would not detach/arrive a monitor
fn notify(monitors: Vec<Monitor>) {
    // Duplicated id's will not cause any issue, however duplicated resolutions/refresh rates are possible
    // They should all be unique anyways. So warn + noop if the sender sent incorrect data
    if has_duplicates(&monitors) {
        warn!("notify(): Duplicate data was detected; update aborted");
        return;
    }

    let adapter = ADAPTER.get().unwrap().0.as_ptr();

    let mut lock = MONITOR_MODES.lock().unwrap();

    // Remove monitors from internal list which are missing from the provided list

    lock.retain_mut(|mon| {
        let id = mon.data.id;
        let found = monitors.iter().any(|m| m.id == id);

        // if it doesn't exist, then add to removal list
        if !found {
            // monitor not found in monitors list, so schedule to remove it
            if let Some(mut obj) = mon.object.take() {
                // remove any monitors scheduled for removal
                let obj = unsafe { obj.as_mut() };
                if let Err(e) = unsafe { IddCxMonitorDeparture(obj) } {
                    error!("Failed to remove monitor: {e:?}");
                }
            }
        }

        found
    });

    let should_arrive = monitors
        .into_iter()
        .map(|monitor| {
            let id = monitor.id;

            let should_arrive;

            let cur_mon = lock.iter_mut().find(|mon| mon.data.id == id);

            if let Some(mon) = cur_mon {
                let modes_changed = mon.data.modes != monitor.modes;

                #[allow(clippy::nonminimal_bool)]
                {
                    should_arrive =
                        // previously was disabled, and it was just enabled
                        (!mon.data.enabled && monitor.enabled) ||
                        // OR monitor is enabled and the display modes changed
                        (monitor.enabled && modes_changed) ||
                        // OR monitor is enabled and the monitor was disconnected
                        (monitor.enabled && mon.object.is_none());
                }

                // should only detach if modes changed, or if state is false
                if modes_changed || !monitor.enabled {
                    if let Some(mut obj) = mon.object.take() {
                        let obj = unsafe { obj.as_mut() };
                        if let Err(e) = unsafe { IddCxMonitorDeparture(obj) } {
                            error!("Failed to remove monitor: {e:?}");
                        }
                    }
                }

                // update monitor data
                mon.data = monitor;
            } else {
                should_arrive = monitor.enabled;

                lock.push(MonitorObject {
                    object: None,
                    data: monitor,
                });
            }

            (id, should_arrive)
        })
        .collect::<Vec<_>>();

    // context.create_monitor locks again, so this avoids deadlock
    drop(lock);

    let cb = |context: &mut DeviceContext| {
        // arrive any monitors that need arriving
        for (id, arrive) in should_arrive {
            if arrive {
                if let Err(e) = context.create_monitor(id) {
                    error!("Failed to create monitor: {e:?}");
                }
            }
        }
    };

    unsafe {
        DeviceContext::get_mut(adapter.cast(), cb).unwrap();
    }
}

fn remove_all() {
    let mut lock = MONITOR_MODES.lock().unwrap();

    for monitor in lock.drain(..) {
        if let Some(mut monitor_object) = monitor.object {
            let obj = unsafe { monitor_object.as_mut() };
            if let Err(e) = unsafe { IddCxMonitorDeparture(obj) } {
                error!("Failed to remove monitor: {e:?}");
            }
        }
    }
}

fn remove(ids: &[u32]) {
    let mut lock = MONITOR_MODES.lock().unwrap();

    for &id in ids {
        lock.retain_mut(|monitor| {
            if id == monitor.data.id {
                if let Some(mut monitor_object) = monitor.object.take() {
                    let obj = unsafe { monitor_object.as_mut() };
                    if let Err(e) = unsafe { IddCxMonitorDeparture(obj) } {
                        error!("Failed to remove monitor: {e:?}");
                    }
                }

                false
            } else {
                true
            }
        });
    }
}

pub trait FlattenModes {
    fn flatten(&self) -> impl Iterator<Item = ModeItem>;
}

#[derive(Copy, Clone)]
pub struct ModeItem {
    pub width: Dimen,
    pub height: Dimen,
    pub refresh_rate: RefreshRate,
}

/// Takes a slice of modes and creates a flattened structure that can be iterated over
impl FlattenModes for Vec<Mode> {
    fn flatten(&self) -> impl Iterator<Item = ModeItem> {
        self.iter().flat_map(|m| {
            m.refresh_rates.iter().map(|&rr| ModeItem {
                width: m.width,
                height: m.height,
                refresh_rate: rr,
            })
        })
    }
}
