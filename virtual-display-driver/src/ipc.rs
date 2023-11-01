use std::{
    ops::ControlFlow,
    ptr::NonNull,
    sync::{Mutex, OnceLock},
    thread,
};

use driver_ipc::{Command, Monitor};
use log::warn;
use wdf_umdf::IddCxMonitorDeparture;
use wdf_umdf_sys::{IDDCX_ADAPTER__, IDDCX_MONITOR__};
use win_pipes::NamedPipeServerOptions;
use winreg::{
    enums::{HKEY_LOCAL_MACHINE, KEY_READ},
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
            add(monitors);
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
                    Command::DriverAdd(monitors) => add(monitors),

                    Command::DriverRemove(ids) => {
                        if let Some(ControlFlow::Continue(_)) = remove(ids) {
                            continue;
                        }
                    }

                    Command::DriverRemoveAll => {
                        if let Some(ControlFlow::Continue(_)) = remove_all() {
                            continue;
                        }
                    }

                    Command::RequestState => {
                        let lock = MONITOR_MODES.get().unwrap().lock().unwrap();
                        let monitors = lock.iter().map(|m| m.monitor.clone()).collect::<Vec<_>>();
                        let command = Command::ReplyState(monitors);

                        let Ok(serialized) = serde_json::to_string(&command) else {
                            continue;
                        };

                        _ = client.write(serialized.as_bytes());
                    }

                    // Everything else is an invalid command
                    _ => continue,
                }
            }
        }
    });
}

fn get_data() -> Vec<Monitor> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = r"SOFTWARE\VirtualDisplayDriver";

    let Ok(driver_settings) = hklm.open_subkey_with_flags(key, KEY_READ) else {
        return Vec::new();
    };

    driver_settings
        .get_value::<String, _>("data")
        .map(|data| serde_json::from_str::<Vec<Monitor>>(&data).unwrap_or_default())
        .unwrap_or_default()
}

fn add(monitors: Vec<Monitor>) {
    let adapter = ADAPTER.get().unwrap().0.as_ptr();

    unsafe {
        DeviceContext::get_mut(adapter as *mut _, |context| {
            for monitor in monitors {
                let id = monitor.id;

                {
                    let mut lock = MONITOR_MODES.get().unwrap().lock().unwrap();

                    // if this monitor index is already in, do not add it, no-op it
                    if lock.iter().any(|m| m.monitor.id == id) {
                        warn!("Cannot add monitor {id}, because it is already added");
                        continue;
                    }

                    lock.push(MonitorObject {
                        monitor_object: None,
                        monitor,
                    });
                }

                context.create_monitor(id);
            }
        })
        .unwrap();
    }
}

fn remove_all() -> Option<ControlFlow<()>> {
    let mut lock = MONITOR_MODES.get().unwrap().lock().unwrap();

    for monitor in lock.drain(..) {
        let Some(mut monitor_object) = monitor.monitor_object else {
            return Some(ControlFlow::Continue(()));
        };

        unsafe {
            IddCxMonitorDeparture(monitor_object.as_mut()).unwrap();
        }
    }

    None
}

fn remove(ids: Vec<u32>) -> Option<ControlFlow<()>> {
    let mut lock = MONITOR_MODES.get().unwrap().lock().unwrap();

    let mut to_remove = Vec::new();

    for &id in ids.iter() {
        for (i, monitor) in lock.iter().enumerate() {
            if id == monitor.monitor.id {
                to_remove.push(i);

                let Some(mut monitor_object) = monitor.monitor_object else {
                    return Some(ControlFlow::Continue(()));
                };

                unsafe {
                    IddCxMonitorDeparture(monitor_object.as_mut()).unwrap();
                }
            }
        }
    }

    for r_id in to_remove {
        lock.remove(r_id);
    }

    None
}
