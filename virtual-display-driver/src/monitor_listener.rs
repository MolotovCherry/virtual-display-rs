use std::{
    io::Read,
    net::TcpListener,
    ops::ControlFlow,
    ptr::NonNull,
    sync::{Mutex, OnceLock},
    thread,
};

use driver_ipc::{DriverCommand, Monitor};
use log::{error, warn};
use wdf_umdf::IddCxMonitorDeparture;
use wdf_umdf_sys::{IDDCX_ADAPTER__, IDDCX_MONITOR__};
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
        let (port, monitors) = get_data();

        // add default monitors saved in registry
        if !monitors.is_empty() {
            add(monitors);
        }

        let connect = || {
            let Ok(listener) = TcpListener::bind(format!("127.0.0.1:{port}")) else {
                return None;
            };

            Some(listener)
        };

        let listener = connect();

        if let Some(listener) = listener {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else {
                    error!("Failed to get stream");
                    break;
                };

                loop {
                    // u64 length
                    let mut length = [0u8; 8];

                    if stream.read_exact(&mut length).is_err() {
                        // client disconnected
                        break;
                    }

                    let length = u64::from_le_bytes(length);

                    // 1mb limit (to prevent attacks)
                    if length >= 1024 * 1024 {
                        warn!("Client requested allocation of {length}; this has been blocked");
                        continue;
                    }

                    // create zeroed vector the size of len
                    let mut buffer = vec![0; length as usize];

                    if stream.read_exact(&mut buffer).is_err() {
                        error!("Received data could not be read");
                        continue;
                    };

                    let Ok(data) = String::from_utf8(buffer) else {
                        error!("Received data is not a valid string");
                        continue;
                    };

                    let Ok(data) = serde_json::from_str::<DriverCommand>(&data) else {
                        error!("Received data could not be deserialized");
                        continue;
                    };

                    match data {
                        DriverCommand::Add(monitors) => add(monitors),

                        DriverCommand::Remove(ids) => {
                            if let Some(ControlFlow::Continue(_)) = remove(ids) {
                                continue;
                            }
                        }

                        DriverCommand::RemoveAll => {
                            if let Some(ControlFlow::Continue(_)) = remove_all() {
                                continue;
                            }
                        }
                    }
                }
            }
        }
    });
}

fn get_data() -> (u32, Vec<Monitor>) {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = "SOFTWARE\\VirtualDisplayDriver";
    let port = 23112u32;

    let Ok(driver_settings) = hklm.open_subkey_with_flags(key, KEY_READ) else {
        return (port, Vec::new());
    };

    let port = driver_settings.get_value("port").unwrap_or(port);

    let data = driver_settings
        .get_value::<String, _>("data")
        .map(|data| serde_json::from_str::<Vec<Monitor>>(&data).unwrap_or_default())
        .unwrap_or_default();

    (port, data)
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
