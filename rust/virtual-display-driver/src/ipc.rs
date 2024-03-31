use std::{
    io::Write,
    mem::size_of,
    ptr::{addr_of_mut, NonNull},
    sync::{Mutex, OnceLock},
    thread,
};

use driver_ipc::{
    Command, Dimen, DriverCommand, Mode, Monitor, RefreshRate, ReplyCommand, RequestCommand,
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
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_READ},
    RegKey,
};

use crate::context::DeviceContext;

pub static ADAPTER: OnceLock<AdapterObject> = OnceLock::new();
pub static MONITOR_MODES: OnceLock<Mutex<Vec<MonitorObject>>> = OnceLock::new();

#[derive(Debug)]
pub struct AdapterObject(pub NonNull<IDDCX_ADAPTER__>);
unsafe impl Sync for AdapterObject {}
unsafe impl Send for AdapterObject {}

#[derive(Debug)]
pub struct MonitorObject {
    pub monitor_object: Option<NonNull<IDDCX_MONITOR__>>,
    pub monitor: Monitor,
}
unsafe impl Sync for MonitorObject {}
unsafe impl Send for MonitorObject {}

pub fn startup() {
    MONITOR_MODES.set(Mutex::new(Vec::new())).unwrap();

    thread::spawn(move || {
        let monitors = get_data();

        // add default monitors saved in registry
        if !monitors.is_empty() {
            notify(monitors);
        }

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

        let server = NamedPipeServerOptions::new("virtualdisplaydriver")
            .reject_remote()
            .read_message()
            .write_message()
            .access_duplex()
            .first_pipe_instance()
            .max_instances(1)
            .in_buffer_size(4096)
            .out_buffer_size(4096)
            .security_attributes(&sa)
            .wait()
            .create()
            .unwrap();

        for client in server.incoming() {
            let Ok((reader, mut writer)) = client else {
                // errors are safe to continue on
                continue;
            };

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
                        DriverCommand::Notify(monitors) => notify(monitors),

                        DriverCommand::Remove(ids) => remove(&ids),

                        DriverCommand::RemoveAll => remove_all(),

                        _ => continue,
                    },

                    // request commands
                    Command::Request(RequestCommand::State) => {
                        let lock = MONITOR_MODES.get().unwrap().lock().unwrap();
                        let monitors = lock.iter().map(|m| m.monitor.clone()).collect::<Vec<_>>();
                        let command = ReplyCommand::State(monitors);

                        let Ok(serialized) = serde_json::to_string(&command) else {
                            continue;
                        };

                        _ = writer.write_all(serialized.as_bytes());
                    }

                    // Everything else is an invalid command
                    _ => continue,
                }
            }
        }
    });
}

fn get_data() -> Vec<Monitor> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let key = r"SOFTWARE\VirtualDisplayDriver";

    let Ok(driver_settings) = hklm.open_subkey_with_flags(key, KEY_READ) else {
        return Vec::new();
    };

    driver_settings
        .get_value::<String, _>("data")
        .map(|data| serde_json::from_str(&data).unwrap_or_default())
        .unwrap_or_default()
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
/// Only detaches/reattaches if required in order to update monitor state in OS.
/// e.g. only a name update would not detach/arrive a monitor
fn notify(monitors: Vec<Monitor>) {
    // Duplicated id's will not cause any issue, however duplicated resolutions/refresh rates are possible
    // They should all be unique anyways. So warn + noop if the sender sent incorrect data
    if has_duplicates(&monitors) {
        warn!("notify(): Duplicate data was detected; update aborted");
        return;
    }

    let adapter = ADAPTER.get().unwrap().0.as_ptr();

    // Remove monitors from internal list which are missing from the provided list
    let removed_monitors = {
        let mut lock = MONITOR_MODES.get().unwrap().lock().unwrap();

        let mut remove = Vec::new();
        lock.retain_mut(|mon| {
            let id = mon.monitor.id;
            let found = monitors.iter().any(|m| m.id == id);

            // if it doesn't exist, then add to removal list
            if !found {
                // monitor not found in monitors list, so schedule to remove it
                if let Some(obj) = mon.monitor_object.take() {
                    remove.push(obj);
                }
            }

            found
        });

        remove
    };

    let should_arrive = monitors.into_iter().map(|monitor| {
        let id = monitor.id;

        let should_arrive;

        let mut lock = MONITOR_MODES.get().unwrap().lock().unwrap();

        let cur_mon = lock
            .iter_mut()
            .enumerate()
            .find(|(_, mon)| mon.monitor.id == id);

        if let Some((i, mon)) = cur_mon {
            let modes_changed = mon.monitor.modes != monitor.modes;

            #[allow(clippy::nonminimal_bool)]
            {
                should_arrive =
                        // previously was disabled, and it was just enabled
                        (!mon.monitor.enabled && monitor.enabled) ||
                        // OR monitor is enabled and the display modes changed
                        (monitor.enabled && modes_changed);
            }

            // should only detach if modes changed, or if state is false
            if modes_changed || !monitor.enabled {
                if let Some(mut obj) = mon.monitor_object.take() {
                    let obj = unsafe { obj.as_mut() };
                    if let Err(e) = unsafe { IddCxMonitorDeparture(obj) } {
                        error!("Failed to remove monitor: {e:?}");
                    }
                }
            }

            // replace existing item with new object
            lock[i] = MonitorObject {
                monitor_object: mon.monitor_object,
                monitor,
            };
        } else {
            should_arrive = monitor.enabled;

            lock.push(MonitorObject {
                monitor_object: None,
                monitor,
            });
        }

        (id, should_arrive)
    });

    let cb = |context: &mut DeviceContext| {
        // arrive any monitors that need arriving
        for (id, arrive) in should_arrive {
            if arrive {
                if let Err(e) = context.create_monitor(id) {
                    error!("Failed to create monitor: {e:?}");
                }
            }
        }

        // remove any monitors scheduled for removal
        for mut obj in removed_monitors {
            let obj = unsafe { obj.as_mut() };
            if let Err(e) = unsafe { IddCxMonitorDeparture(obj) } {
                error!("Failed to remove monitor: {e:?}");
            }
        }
    };

    unsafe {
        DeviceContext::get_mut(adapter.cast(), cb).unwrap();
    }
}

fn remove_all() {
    let mut lock = MONITOR_MODES.get().unwrap().lock().unwrap();

    for monitor in lock.drain(..) {
        if let Some(mut monitor_object) = monitor.monitor_object {
            let obj = unsafe { monitor_object.as_mut() };
            if let Err(e) = unsafe { IddCxMonitorDeparture(obj) } {
                error!("Failed to remove monitor: {e:?}");
            }
        }
    }
}

fn remove(ids: &[u32]) {
    let mut lock = MONITOR_MODES.get().unwrap().lock().unwrap();

    for &id in ids {
        lock.retain_mut(|monitor| {
            if id == monitor.monitor.id {
                if let Some(mut monitor_object) = monitor.monitor_object.take() {
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
