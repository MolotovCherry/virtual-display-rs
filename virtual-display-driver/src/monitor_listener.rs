use std::{
    io::Read,
    net::TcpListener,
    ops::ControlFlow,
    ptr::NonNull,
    sync::{atomic::Ordering, Mutex, OnceLock},
    thread,
};

use driver_ipc::{DriverCommand, Monitor};
use log::{error, info, warn};
use wdf_umdf::IddCxMonitorDeparture;
use wdf_umdf_sys::{IDDCX_ADAPTER__, IDDCX_MONITOR__};

use crate::device_context::{DeviceContext, FINISHED_INIT};

pub static ADAPTER: OnceLock<AdapterObject> = OnceLock::new();
pub static MONITOR_MODES: OnceLock<Mutex<Vec<MonitorObject>>> = OnceLock::new();

#[derive(Debug)]
pub struct AdapterObject(pub NonNull<IDDCX_ADAPTER__>);
unsafe impl Send for AdapterObject {}
unsafe impl Sync for AdapterObject {}

#[derive(Debug)]
pub struct MonitorObject {
    pub monitor_object: Option<NonNull<IDDCX_MONITOR__>>,
    pub monitor: Monitor,
}

// Safety: Because it is 😁
unsafe impl Send for MonitorObject {}
unsafe impl Sync for MonitorObject {}

/// Warning, this method locks MONITOR_MODES
pub fn monitor_count() -> usize {
    let Some(lock) = MONITOR_MODES.get() else {
        // not initted yet
        return 0;
    };

    let Ok(lock) = lock.lock() else {
        // lock problem, there's no data to return
        return 0;
    };

    lock.len()
}

pub fn start_listener(port: u32) {
    MONITOR_MODES.set(Mutex::new(Vec::new())).unwrap();

    thread::spawn(move || {
        // wait until we can initialize
        while !FINISHED_INIT.load(Ordering::Acquire) {
            // The spin loop is a hint to the CPU that we're waiting, but probably
            // not for very long
            std::hint::spin_loop();
        }

        info!("listening on 127.0.0.1:{port}");

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

fn add(monitors: Vec<Monitor>) {
    let adapter = ADAPTER.get().unwrap().0.as_ptr();
    unsafe {
        DeviceContext::get_mut(adapter as *mut _, |context| {
            for monitor in monitors {
                let id = monitor.id;

                {
                    let Some(lock) = MONITOR_MODES.get() else {
                        return;
                    };

                    let mut lock = lock.lock().unwrap();

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
    let Some(lock) = MONITOR_MODES.get() else {
        // not initted yet
        return Some(ControlFlow::Continue(()));
    };

    let Ok(mut lock) = lock.lock() else {
        // lock problem, there's no data to return
        return Some(ControlFlow::Continue(()));
    };

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
    let Some(lock) = MONITOR_MODES.get() else {
        // not initted yet
        return Some(ControlFlow::Continue(()));
    };

    let Ok(mut lock) = lock.lock() else {
        // lock problem, there's no data to return
        return Some(ControlFlow::Continue(()));
    };

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
