use std::{
    ptr::NonNull,
    sync::{Mutex, OnceLock},
    thread,
};

use driver_ipc::{Command, Monitor};
use wdf_umdf::IddCxMonitorDeparture;
use wdf_umdf_sys::{IDDCX_ADAPTER__, IDDCX_MONITOR__};
use win_pipes::NamedPipeServerOptions;
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

/// WARNING: Locks MONITOR_MODES, don't call if already locked or deadlock happens
pub fn monitor_count() -> usize {
    MONITOR_MODES.get().unwrap().lock().unwrap().len()
}

pub fn startup() {
    MONITOR_MODES.set(Mutex::new(Vec::new())).unwrap();

    thread::spawn(move || {
        let monitors = get_data();

        // add default monitors saved in registry
        if !monitors.is_empty() {
            add_or_update_monitors(monitors);
        }

        let server = NamedPipeServerOptions::new(r"\\.\pipe\virtualdisplaydriver")
            .reject_remote()
            .read_message()
            .write_message()
            .access_duplex()
            .first_pipe_instance()
            .max_instances(1)
            .in_buffer_size(4096)
            .wait()
            .create()
            .unwrap();

        for client in server.incoming() {
            let Ok(client) = client else {
                // errors are safe to continue on
                continue;
            };

            for data in client.iter_full() {
                let Ok(msg) = std::str::from_utf8(&data) else {
                    _ = server.disconnect();
                    continue;
                };

                let Ok(msg) = serde_json::from_str::<Command>(msg) else {
                    _ = server.disconnect();
                    continue;
                };

                match msg {
                    Command::DriverAddOrUpdateMonitor(monitors) => add_or_update_monitors(monitors),

                    Command::DriverRemoveMonitor(ids) => remove(ids),

                    Command::DriverRemoveAllMonitors => remove_all(),

                    Command::RequestState => {
                        let lock = MONITOR_MODES.get().unwrap().lock().unwrap();
                        let monitors = lock.iter().map(|m| m.monitor.clone()).collect::<Vec<_>>();
                        let command = Command::ReplyState(monitors);

                        let Ok(serialized) = serde_json::to_string(&command) else {
                            continue;
                        };

                        _ = client.write(serialized.as_bytes());
                    }

                    Command::DriverUpdateName(monitor) => update_name(monitor),

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

/// This exists solely to update the label without changing the monitor itself
fn update_name(monitor: Monitor) {
    let mut lock = MONITOR_MODES.get().unwrap().lock().unwrap();

    if let Some(found_monitor) = lock.iter_mut().find(|m| m.monitor.id == monitor.id) {
        found_monitor.monitor.name = monitor.name;
    }
}

/// Adds if it doesn't exist,
/// Or, if it does exist, updates it by detaching, update, and re-attaching
fn add_or_update_monitors(monitors: Vec<Monitor>) {
    let adapter = ADAPTER.get().unwrap().0.as_ptr();

    unsafe {
        DeviceContext::get_mut(adapter as *mut _, |context| {
            for monitor in monitors {
                let id = monitor.id;

                {
                    let mut lock = MONITOR_MODES.get().unwrap().lock().unwrap();

                    let cur_mon = lock
                        .iter_mut()
                        .enumerate()
                        .find(|(_, mon)| mon.monitor.id == id);

                    if let Some((i, mon)) = cur_mon {
                        // if monitor is already enabled, we should detach it and replace it
                        if let Some(mut obj) = mon.monitor_object.take() {
                            IddCxMonitorDeparture(obj.as_mut()).unwrap();
                        }

                        // replace existing item with new object
                        lock[i] = MonitorObject {
                            monitor_object: None,
                            monitor: monitor.clone(),
                        };
                    } else {
                        lock.push(MonitorObject {
                            monitor_object: None,
                            monitor: monitor.clone(),
                        });
                    }
                }

                if monitor.enabled {
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

fn remove(ids: Vec<u32>) {
    let mut lock = MONITOR_MODES.get().unwrap().lock().unwrap();

    for &id in ids.iter() {
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
