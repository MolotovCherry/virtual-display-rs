use wdf_umdf::WdfFunction;
use wdf_umdf_sys::{
    WdfFunctions_02033, WDFDEVICE_INIT, WDFDRIVER, WDFDRIVER__, WDF_DRIVER_CONFIG,
    WDF_EXECUTION_LEVEL, WDF_OBJECT_ATTRIBUTES, WDF_SYNCHRONIZATION_SCOPE, _DRIVER_OBJECT,
    _UNICODE_STRING,
};
use windows::Win32::Foundation::NTSTATUS;

use crate::popup::{display_popup, MessageBoxIcon};

// See windows::Wdk::System::SystemServices::DRIVER_INITIALIZE
#[no_mangle]
extern "system" fn DriverEntry(
    driver_object: *mut _DRIVER_OBJECT,
    registry_path: *mut _UNICODE_STRING,
) -> NTSTATUS {
    // equivalent to WDF_OBJECT_ATTRIBUTES_INIT
    // https://github.com/microsoft/Windows-Driver-Frameworks/blob/a94b8c30dad524352fab90872aefc83920b98e56/src/publicinc/wdf/umdf/2.27/wdfobject.h#L134
    let mut attributes = WDF_OBJECT_ATTRIBUTES {
        Size: std::mem::size_of::<WDF_OBJECT_ATTRIBUTES>() as u32,
        ExecutionLevel: WDF_EXECUTION_LEVEL::WdfExecutionLevelInheritFromParent,
        SynchronizationScope: WDF_SYNCHRONIZATION_SCOPE::WdfSynchronizationScopeInheritFromParent,
        ..Default::default()
    };

    // equivalent to WDF_DRIVER_CONFIG_INIT
    // https://github.com/microsoft/Windows-Driver-Frameworks/blob/a94b8c30dad524352fab90872aefc83920b98e56/src/publicinc/wdf/umdf/2.23/wdfdriver.h#L131
    let mut config = WDF_DRIVER_CONFIG {
        Size: std::mem::size_of::<WDF_DRIVER_CONFIG>() as u32,
        EvtDriverDeviceAdd: Some(driver_add),
        ..Default::default()
    };

    let res = unsafe {
        WdfFunction!(
            WdfDriverCreate,
            driver_object,
            registry_path,
            &mut attributes,
            &mut config,
            std::ptr::null_mut()
        )
    };

    // if status.is_err() {
    //     display_popup(
    //         "VirtualMonitorDriver",
    //         &format!("The driver failed to load: 0x{:X}", status.to_hresult().0),
    //         MessageBoxIcon::Warning,
    //     );
    // }

    // status
    todo!()
}

extern "C" fn driver_add(driver: *mut WDFDRIVER__, init: *mut WDFDEVICE_INIT) -> i32 {
    todo!()
}
