use std::{
    io::Write,
    mem::{self, size_of},
    ptr::{addr_of_mut, NonNull},
    sync::{Mutex, OnceLock},
    thread,
};

use crossbeam_channel::unbounded;
use driver_ipc::{
    Command, Dimen, DriverCommand, EventCommand, Mode, Monitor, RefreshRate, ReplyCommand,
    RequestCommand,
};
use log::{error, warn};
use wdf_umdf::IddCxMonitorDeparture;
use wdf_umdf_sys::{IDDCX_ADAPTER__, IDDCX_MONITOR__};
use win_pipes::NamedPipeServerOptions;
use windows::Win32::{
    Security::{
        InitializeSecurityDescriptor, SetSecurityDescriptorDacl, PSECURITY_DESCRIPTOR,
        SECURITY_ATTRIBUTES, SECURITY_DESCRIPTOR,
    },
    System::SystemServices::SECURITY_DESCRIPTOR_REVISION,
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

#[allow(clippy::too_many_lines)]
pub fn startup() {
    thread::spawn(move || {
        // These security attributes will allow anyone access, so local account does not need admin privileges to use it

        let mut sd = SECURITY_DESCRIPTOR::default();

        unsafe {
            InitializeSecurityDescriptor(
                PSECURITY_DESCRIPTOR(addr_of_mut!(sd).cast()),
                SECURITY_DESCRIPTOR_REVISION,
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

        let sa = SECURITY_ATTRIBUTES {
            #[allow(clippy::cast_possible_truncation)]
            nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: addr_of_mut!(sd).cast(),
            bInheritHandle: false.into(),
        };

        let (notify_s, notify_r) = unbounded();
        let mut client_id = 0usize;

        loop {
            let server = NamedPipeServerOptions::new("virtualdisplaydriver")
                .reject_remote()
                .read_message()
                .write_message()
                .access_duplex()
                .unlimited_instances()
                .in_buffer_size(4096)
                .out_buffer_size(4096)
                .security_attributes(&sa)
                .wait()
                .create()
                .unwrap();

            let Ok((reader, mut writer)) = server.connect() else {
                continue;
            };

            let (notify_s, notify_r) = (notify_s.clone(), notify_r.clone());
            client_id = client_id.wrapping_add(1);

            thread::spawn(move || {
                // process changed events
                let mut notify_writer = writer.clone();
                thread::spawn(move || {
                    while let Ok((id, data)) = notify_r.recv() {
                        // this is the same client, so ignore it
                        if id == client_id {
                            continue;
                        }

                        let command = EventCommand::Changed(data);
                        let Ok(serialized) = serde_json::to_string(&command) else {
                            error!("Command::Request - failed to serialize reply");
                            continue;
                        };

                        _ = notify_writer.write_all(serialized.as_bytes());
                    }

                    mem::forget(notify_writer);
                });

                for data in reader.iter_read_full() {
                    let Ok(msg) = std::str::from_utf8(&data) else {
                        _ = server.disconnect();
                        continue;
                    };

                    let Ok(msg) = serde_json::from_str::<Command>(msg) else {
                        _ = server.disconnect();
                        continue;
                    };

                    #[allow(clippy::match_wildcard_for_single_variants)]
                    match msg {
                        // driver commands
                        Command::Driver(cmd) => match cmd {
                            DriverCommand::Notify(monitors) => {
                                notify(monitors.clone());
                                _ = notify_s.send((client_id, monitors));
                            }

                            DriverCommand::Remove(ids) => {
                                remove(&ids);

                                let lock = MONITOR_MODES.lock().unwrap();
                                let monitors = lock.iter().map(|m| m.data.clone()).collect();
                                _ = notify_s.send((client_id, monitors));
                            }

                            DriverCommand::RemoveAll => {
                                remove_all();
                                _ = notify_s.send((client_id, Vec::new()));
                            }

                            _ => continue,
                        },

                        // request commands
                        Command::Request(RequestCommand::State) => {
                            let lock = MONITOR_MODES.lock().unwrap();
                            let monitors = lock.iter().map(|m| m.data.clone()).collect();
                            let command = ReplyCommand::State(monitors);

                            let Ok(serialized) = serde_json::to_string(&command) else {
                                error!("Command::Request - failed to serialize reply");
                                continue;
                            };

                            _ = writer.write_all(serialized.as_bytes());
                        }

                        // Everything else is an invalid command
                        _ => continue,
                    }
                }
            });
        }
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
