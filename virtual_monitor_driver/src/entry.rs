use windows::{
    Wdk::Foundation::DRIVER_OBJECT,
    Win32::Foundation::{NTSTATUS, UNICODE_STRING},
};

use crate::{
    popup::{display_popup, MessageBoxIcon},
    umdf::{
        WdfFunctions_02033, _WDF_EXECUTION_LEVEL_WdfExecutionLevelInheritFromParent,
        _WDF_SYNCHRONIZATION_SCOPE_WdfSynchronizationScopeInheritFromParent, WDFDEVICE_INIT,
        WDFDRIVER, WDFDRIVER__, WDF_DRIVER_CONFIG, WDF_OBJECT_ATTRIBUTES,
    },
};

// See windows::Wdk::System::SystemServices::DRIVER_INITIALIZE
#[no_mangle]
extern "system" fn DriverEntry(
    driver_object: *mut DRIVER_OBJECT,
    registry_path: *mut UNICODE_STRING,
) -> NTSTATUS {
    // equivalent to WDF_OBJECT_ATTRIBUTES_INIT
    // https://github.com/microsoft/Windows-Driver-Frameworks/blob/a94b8c30dad524352fab90872aefc83920b98e56/src/publicinc/wdf/umdf/2.27/wdfobject.h#L134
    let mut attributes = WDF_OBJECT_ATTRIBUTES {
        Size: std::mem::size_of::<WDF_OBJECT_ATTRIBUTES>() as u32,
        ExecutionLevel: _WDF_EXECUTION_LEVEL_WdfExecutionLevelInheritFromParent,
        SynchronizationScope: _WDF_SYNCHRONIZATION_SCOPE_WdfSynchronizationScopeInheritFromParent,
        ..Default::default()
    };

    // equivalent to WDF_DRIVER_CONFIG_INIT
    // https://github.com/microsoft/Windows-Driver-Frameworks/blob/a94b8c30dad524352fab90872aefc83920b98e56/src/publicinc/wdf/umdf/2.23/wdfdriver.h#L131
    let mut config = WDF_DRIVER_CONFIG {
        Size: std::mem::size_of::<WDF_DRIVER_CONFIG>() as u32,
        EvtDriverDeviceAdd: Some(driver_add),
        ..Default::default()
    };

    let status = unsafe {
        WdfFunctions_02033;
        // WdfDriverCreate(
        //     driver_object,
        //     registry_path,
        //     &mut attributes,
        //     &mut config,
        //     std::ptr::null_mut(),
        // )
    };

    if status.is_err() {
        display_popup(
            "VirtualMonitorDriver",
            &format!("The driver failed to load: 0x{:X}", status.to_hresult().0),
            MessageBoxIcon::Warning,
        );
    }

    status
}

extern "C" fn driver_add(driver: *mut WDFDRIVER__, init: *mut WDFDEVICE_INIT) -> i32 {
    todo!()
}
