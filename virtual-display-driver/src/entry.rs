use log::{error, Level};
use wdf_umdf::{
    IddCxDeviceInitConfig, IddCxDeviceInitialize, IntoHelper, WdfDeviceCreate,
    WdfDeviceInitSetPnpPowerEventCallbacks, WdfDriverCreate,
};
use wdf_umdf_sys::{
    IDD_CX_CLIENT_CONFIG, NTSTATUS, WDFDEVICE_INIT, WDFDRIVER__, WDFOBJECT, WDF_DRIVER_CONFIG,
    WDF_OBJECT_ATTRIBUTES, WDF_PNPPOWER_EVENT_CALLBACKS, _DRIVER_OBJECT, _UNICODE_STRING,
};
use windows::Win32::Foundation::STATUS_UNSUCCESSFUL;

use crate::indirect_device_context::{
    adapter_commit_modes, adapter_init_finished, assign_swap_chain, device_d0_entry,
    monitor_get_default_modes, monitor_query_modes, parse_monitor_description, unassign_swap_chain,
    IndirectDeviceContext, WdfObjectIndirectDeviceContext,
};

//
// Our driver's entry point
// See windows::Wdk::System::SystemServices::DRIVER_INITIALIZE
//
#[no_mangle]
extern "C-unwind" fn DriverEntry(
    driver_object: *mut _DRIVER_OBJECT,
    registry_path: *mut _UNICODE_STRING,
) -> NTSTATUS {
    let status = windebug_logger::init_with_level(if cfg!(debug_assertions) {
        Level::Debug
    } else {
        Level::Info
    })
    .map(|_| NTSTATUS(0))
    .unwrap_or(NTSTATUS(STATUS_UNSUCCESSFUL.0));

    if !status.is_success() {
        return status;
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
        return 0xC0000225u32.into();
    };

    config.EvtIddCxAdapterInitFinished = Some(adapter_init_finished);

    config.EvtIddCxParseMonitorDescription = Some(parse_monitor_description);
    config.EvtIddCxMonitorGetDefaultDescriptionModes = Some(monitor_get_default_modes);
    config.EvtIddCxMonitorQueryTargetModes = Some(monitor_query_modes);
    config.EvtIddCxAdapterCommitModes = Some(adapter_commit_modes);
    config.EvtIddCxMonitorAssignSwapChain = Some(assign_swap_chain);
    config.EvtIddCxMonitorUnassignSwapChain = Some(unassign_swap_chain);

    let status = unsafe { IddCxDeviceInitConfig(&mut *init, &config) };
    if let Err(status) = status {
        return status.into();
    }

    let mut attributes = WDF_OBJECT_ATTRIBUTES::init_context_type(unsafe {
        WdfObjectIndirectDeviceContext::get_type_info()
    });

    attributes.EvtCleanupCallback = Some(event_cleanup);

    let mut device = std::ptr::null_mut();

    let status = unsafe { WdfDeviceCreate(&mut init, Some(&mut attributes), &mut device) };
    if let Err(status) = status {
        return status.into();
    }

    let status = unsafe { IddCxDeviceInitialize(device) };
    if let Err(status) = status {
        return status.into();
    }

    let context = IndirectDeviceContext::new(device);

    unsafe { WdfObjectIndirectDeviceContext::init(device as WDFOBJECT, context).into_status() }
}

unsafe extern "C-unwind" fn event_cleanup(wdf_object: WDFOBJECT) {
    _ = WdfObjectIndirectDeviceContext::drop(wdf_object);
}
