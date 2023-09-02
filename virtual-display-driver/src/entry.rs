use wdf_umdf::{WdfCall, WdfDriverCreate};
use wdf_umdf_sys::{
    NTSTATUS, WDFDEVICE_INIT, WDFDRIVER__, WDF_DRIVER_CONFIG, WDF_OBJECT_ATTRIBUTES,
    _DRIVER_OBJECT, _UNICODE_STRING,
};

use crate::popup::{display_popup, MessageBoxIcon};

// See windows::Wdk::System::SystemServices::DRIVER_INITIALIZE
#[no_mangle]
extern "system" fn DriverEntry(
    driver_object: *mut _DRIVER_OBJECT,
    registry_path: *mut _UNICODE_STRING,
) -> NTSTATUS {
    let mut attributes = WDF_OBJECT_ATTRIBUTES::init();

    let mut config = WDF_DRIVER_CONFIG::init(Some(driver_add));

    let status = unsafe {
        WdfDriverCreate(
            driver_object,
            registry_path,
            Some(&mut attributes),
            &mut config,
            None,
        )
    }
    .unwrap_or(0xC0000225u32.into());

    if !status.is_success() {
        display_popup(
            "VirtualMonitorDriver",
            &format!("The driver failed to load: {status}"),
            MessageBoxIcon::Error,
        );
    }

    status
}

extern "C" fn driver_add(driver: *mut WDFDRIVER__, init: *mut WDFDEVICE_INIT) -> i32 {
    todo!()
}
