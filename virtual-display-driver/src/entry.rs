use wdf_umdf::WdfDriverCreate;
use wdf_umdf_sys::{
    NTSTATUS, WDFDEVICE, WDFDEVICE_INIT, WDFDRIVER__, WDF_DRIVER_CONFIG, WDF_OBJECT_ATTRIBUTES,
    WDF_PNPPOWER_EVENT_CALLBACKS, WDF_POWER_DEVICE_STATE, _DRIVER_OBJECT, _UNICODE_STRING,
};

// See windows::Wdk::System::SystemServices::DRIVER_INITIALIZE
#[no_mangle]
extern "system" fn DriverEntry(
    driver_object: *mut _DRIVER_OBJECT,
    registry_path: *mut _UNICODE_STRING,
) -> NTSTATUS {
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
    .unwrap_or(0xC0000225u32.into())
    }

    status
}

extern "C" fn driver_add(driver: *mut WDFDRIVER__, init: *mut WDFDEVICE_INIT) -> i32 {
    todo!()
}
