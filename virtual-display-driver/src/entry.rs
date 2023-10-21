use std::time::{Duration, Instant};

use driver_ipc::Monitor;
use log::{error, info, LevelFilter};
use wdf_umdf::{
    IddCxDeviceInitConfig, IddCxDeviceInitialize, IntoHelper, WdfDeviceAllocAndQueryPropertyEx,
    WdfDeviceCreate, WdfDeviceInitSetPnpPowerEventCallbacks, WdfDeviceSetFailed, WdfDriverCreate,
    WdfMemoryGetBuffer, WdfObjectDelete,
};
use wdf_umdf_sys::{
    DEVPROPTYPE, DEVPROP_TYPE_STRING, IDD_CX_CLIENT_CONFIG, NTSTATUS, WDFDEVICE, WDFDEVICE_INIT,
    WDFDRIVER__, WDFOBJECT, WDF_DEVICE_FAILED_ACTION, WDF_DRIVER_CONFIG, WDF_NO_OBJECT_ATTRIBUTES,
    WDF_OBJECT_ATTRIBUTES, WDF_PNPPOWER_EVENT_CALLBACKS, _DRIVER_OBJECT, _UNICODE_STRING,
    _WDF_DEVICE_PROPERTY_DATA,
};
use windows::{
    Wdk::Foundation::NonPagedPoolNx, Win32::Devices::Properties::DEVPKEY_Device_MatchingDeviceId,
};
use winreg::{
    enums::{HKEY_LOCAL_MACHINE, KEY_READ},
    RegKey,
};

use crate::callbacks::{
    adapter_commit_modes, adapter_init_finished, assign_swap_chain, device_d0_entry,
    monitor_get_default_modes, monitor_query_modes, parse_monitor_description, unassign_swap_chain,
};
use crate::{device_context::DeviceContext, helpers::Sendable};

//
// Our driver's entry point
// See windows::Wdk::System::SystemServices::DRIVER_INITIALIZE
//
#[no_mangle]
extern "C-unwind" fn DriverEntry(
    driver_object: *mut _DRIVER_OBJECT,
    registry_path: *mut _UNICODE_STRING,
) -> NTSTATUS {
    // During system bootup, `RegisterEventSourceW` fails and causes the driver to not bootup
    // Pretty unfortunate, therefore, we will run this on a thread until it succeeds and let the rest of
    // the driver start. I know this is suboptimal considering it's our main code to catch panics.
    //
    // It always starts immediately when the computer is already booted up.
    // If you have a better solution, please by all means open an issue report
    let init_log = || {
        winlog::init(
            "VirtualDisplayDriver",
            if cfg!(debug_assertions) {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            },
        )
        .map(|_| NTSTATUS::STATUS_SUCCESS)
        .unwrap_or(NTSTATUS::STATUS_UNSUCCESSFUL)
    };

    let status = init_log();

    if !status.is_success() {
        // Okay, let's try another method then
        let device = unsafe { Sendable::new(driver_object) };

        std::thread::spawn(move || {
            #[allow(clippy::redundant_locals)]
            let device = device;
            let time_waited = Instant::now();
            // 5 minutes
            let timeout_duration = Duration::from_secs(60 * 5);
            // in ms
            let sleep_for = 500;

            loop {
                let status = init_log();
                std::thread::sleep(Duration::from_millis(sleep_for));

                // if it succeeds, great. if it didn't conclude after 5 minutes
                // Surely a users system is booted up before then?
                let timedout = time_waited.elapsed() >= timeout_duration;
                if status.is_success() || timedout {
                    if timedout {
                        // Service took too long to start. Unfortunately, there is no way to log this failure
                        unsafe {
                            _ = WdfDeviceSetFailed(
                                *device as *mut _,
                                WDF_DEVICE_FAILED_ACTION::WdfDeviceFailedNoRestart,
                            );
                        }
                    } else {
                        info!(
                            "Service took {} seconds to start",
                            time_waited.elapsed().as_secs()
                        );
                    }

                    break;
                }
            }
        });
    }

    // set the panic hook to capture and log panics
    crate::panic::set_hook();

    let mut attributes = WDF_OBJECT_ATTRIBUTES::init();

    let mut config = WDF_DRIVER_CONFIG::init(Some(driver_add));

    unsafe {
        WdfDriverCreate(
            driver_object,
            registry_path,
            Some(&mut attributes),
            &mut config,
            None,
        )
    }
    .into_status()
}

extern "C-unwind" fn driver_add(
    _driver: *mut WDFDRIVER__,
    mut init: *mut WDFDEVICE_INIT,
) -> NTSTATUS {
    let mut callbacks = WDF_PNPPOWER_EVENT_CALLBACKS::init();

    callbacks.EvtDeviceD0Entry = Some(device_d0_entry);

    unsafe {
        _ = WdfDeviceInitSetPnpPowerEventCallbacks(init, &mut callbacks);
    }

    let Some(mut config) = IDD_CX_CLIENT_CONFIG::init() else {
        error!("Failed to create IDD_CX_CLIENT_CONFIG");
        return NTSTATUS::STATUS_NOT_FOUND;
    };

    config.EvtIddCxAdapterInitFinished = Some(adapter_init_finished);

    config.EvtIddCxParseMonitorDescription = Some(parse_monitor_description);
    config.EvtIddCxMonitorGetDefaultDescriptionModes = Some(monitor_get_default_modes);
    config.EvtIddCxMonitorQueryTargetModes = Some(monitor_query_modes);
    config.EvtIddCxAdapterCommitModes = Some(adapter_commit_modes);
    config.EvtIddCxMonitorAssignSwapChain = Some(assign_swap_chain);
    config.EvtIddCxMonitorUnassignSwapChain = Some(unassign_swap_chain);

    let status = unsafe { IddCxDeviceInitConfig(&mut *init, &config) };
    if let Err(e) = status {
        error!("Failed to init iddcx config: {e:?}");
        return e.into();
    }

    let mut attributes =
        WDF_OBJECT_ATTRIBUTES::init_context_type(unsafe { DeviceContext::get_type_info() });

    attributes.EvtCleanupCallback = Some(event_cleanup);

    let mut device = std::ptr::null_mut();

    let status = unsafe { WdfDeviceCreate(&mut init, Some(&mut attributes), &mut device) };
    if let Err(e) = status {
        error!("Failed to create device: {e:?}");
        return e.into();
    }

    let status = unsafe { IddCxDeviceInitialize(device) };
    if let Err(e) = status {
        error!("Failed to init iddcx device: {e:?}");
        return e.into();
    }

    let context = DeviceContext::new(device);

    if let Err(e) = unsafe { context.init(device as WDFOBJECT) } {
        error!("Failed to init context: {e:?}");
        return e.into();
    }

    // get the hardware id for this specific driver instance
    let hardware_id = get_driver_id(device);
    let Ok(hardware_id) = hardware_id else {
        return hardware_id.unwrap_err();
    };

    info!("got hardware id {hardware_id:?}");

    NTSTATUS::STATUS_SUCCESS
}

unsafe extern "C-unwind" fn event_cleanup(wdf_object: WDFOBJECT) {
    _ = DeviceContext::drop(wdf_object);
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

fn get_driver_id(wdfdevice: WDFDEVICE) -> Result<u32, NTSTATUS> {
    let hardware_id = get_hardware_id(wdfdevice)?;
    let Some(id) = hardware_id
        .strip_prefix(r"root\virtualdisplaydriver_")
        .and_then(|n| n.parse::<u32>().ok())
    else {
        return Err(NTSTATUS::STATUS_ASSERTION_FAILURE);
    };

    Ok(id)
}

fn get_hardware_id(wdfdevice: WDFDEVICE) -> Result<String, NTSTATUS> {
    let mut property_data = _WDF_DEVICE_PROPERTY_DATA {
        Size: std::mem::size_of::<_WDF_DEVICE_PROPERTY_DATA>() as u32,
        PropertyKey: &DEVPKEY_Device_MatchingDeviceId as *const _ as *const _,
        ..Default::default()
    };

    let mut property_memory = std::ptr::null_mut();

    let mut _type = DEVPROPTYPE::default();

    let status = unsafe {
        WdfDeviceAllocAndQueryPropertyEx(
            wdfdevice,
            &mut property_data,
            std::mem::transmute(NonPagedPoolNx),
            WDF_NO_OBJECT_ATTRIBUTES!(),
            &mut property_memory,
            &mut _type,
        )
    };

    if let Err(e) = status {
        error!("Failed to alloc and query property: {e}");
        unsafe {
            _ = WdfObjectDelete(property_memory as *mut _);
        }
        return Err(e.into());
    }

    let hardware_id = if _type == DEVPROP_TYPE_STRING {
        let mut size = 0usize;

        let ret = unsafe { WdfMemoryGetBuffer(property_memory, Some(&mut size)) };
        let Ok(data) = ret else {
            let e = ret.unwrap_err();

            error!("Failed to get memory buffer: {e:?}",);
            unsafe {
                _ = WdfObjectDelete(property_memory as *mut _);
            }
            return Err(e.into());
        };

        let slice =
            unsafe { std::slice::from_raw_parts(data as *mut u16, (size / 2).saturating_sub(1)) };
        String::from_utf16_lossy(slice)
    } else {
        error!("got wrong devpkey for hardware id");
        unsafe {
            _ = WdfObjectDelete(property_memory as *mut _);
        }
        return Err(NTSTATUS::STATUS_FAIL_CHECK);
    };

    unsafe {
        _ = WdfObjectDelete(property_memory as *mut _);
    }

    Ok(hardware_id)
}
