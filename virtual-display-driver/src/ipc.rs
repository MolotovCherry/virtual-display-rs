use std::{
    io::Write,
    mem::size_of,
    ptr::{addr_of_mut, NonNull},
    sync::{Mutex, OnceLock},
    thread,
};

use driver_ipc::{Command, Monitor};
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

/// WARNING: Locks `MONITOR_MODES`, don't call if already locked or deadlock happens
pub fn monitor_count() -> usize {
    MONITOR_MODES.get().unwrap().lock().unwrap().len()
}

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
            nLength: u32::try_from(size_of::<SECURITY_ATTRIBUTES>()).unwrap(),
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
                    Command::DriverNotify(monitors) => notify(monitors),

                    Command::DriverRemove(ids) => remove(&ids),

                    Command::DriverRemoveAll => remove_all(),

                    Command::RequestState => {
                        let lock = MONITOR_MODES.get().unwrap().lock().unwrap();
                        let monitors = lock.iter().map(|m| m.monitor.clone()).collect::<Vec<_>>();
                        let command = Command::ReplyState(monitors);

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
        .map(|data| serde_json::from_str::<Vec<Monitor>>(&data).unwrap_or_default())
        .unwrap_or_default()
}

/// Adds if it doesn't exist,
/// Or, if it does exist, updates it by detaching, update, and re-attaching
///
/// Only adds/detaches, if required in order to update monitor state in the OS.
/// e.g. only a name update would not detach/arrive a monitor
fn notify(monitors: Vec<Monitor>) {
    let adapter = ADAPTER.get().unwrap().0.as_ptr();

    unsafe {
        DeviceContext::get_mut(adapter.cast(), |context| {
            for monitor in monitors {
                let id = monitor.id;

                let should_arrive;

                {
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
                                IddCxMonitorDeparture(obj.as_mut()).unwrap();
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
                }

                if should_arrive {
                    context.create_monitor(id);
                }
            }
        })
        .unwrap();
    }
}

fn remove_all() {
    let mut lock = MONITOR_MODES.get().unwrap().lock().unwrap();

    for monitor in lock.drain(..) {
        if let Some(mut monitor_object) = monitor.monitor_object {
            unsafe {
                IddCxMonitorDeparture(monitor_object.as_mut()).unwrap();
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
                    unsafe {
                        IddCxMonitorDeparture(monitor_object.as_mut()).unwrap();
                    }
                }

                false
            } else {
                true
            }
        });
    }
}
