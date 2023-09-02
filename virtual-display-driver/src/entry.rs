use wdf_umdf::{WdfCall, WdfDriverCreate};
use wdf_umdf_sys::{
    WDFDEVICE_INIT, WDFDRIVER__, WDF_DRIVER_CONFIG, WDF_OBJECT_ATTRIBUTES, _DRIVER_OBJECT,
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
    let mut attributes = WDF_OBJECT_ATTRIBUTES::init();

    let mut config = WDF_DRIVER_CONFIG::init(Some(driver_add));

    let res = unsafe {
        WdfDriverCreate(
            driver_object,
            registry_path,
            Some(&mut attributes),
            &mut config,
            None,
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
