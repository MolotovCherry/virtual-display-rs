#![allow(non_snake_case)]

use std::ffi::c_void;

use wdf_umdf_sys::{
    NTSTATUS, PCUNICODE_STRING, PCWDF_OBJECT_CONTEXT_TYPE_INFO, PDRIVER_OBJECT, PWDFDEVICE_INIT,
    PWDF_DRIVER_CONFIG, PWDF_OBJECT_ATTRIBUTES, WDFDEVICE, WDFDRIVER, WDFOBJECT, WDF_NO_HANDLE,
    WDF_NO_OBJECT_ATTRIBUTES, WDF_OBJECT_ATTRIBUTES, _WDF_PNPPOWER_EVENT_CALLBACKS,
};

#[derive(Debug, thiserror::Error)]
pub enum WdfError {
    #[error("{0}")]
    WdfFunctionNotAvailable(&'static str),
    #[error("{0}")]
    CallFailed(NTSTATUS),
}

impl From<WdfError> for () {
    fn from(_: WdfError) -> Self {}
}

impl From<WdfError> for NTSTATUS {
    fn from(value: WdfError) -> Self {
        use WdfError::*;
        match value {
            WdfFunctionNotAvailable(_) => 0xC0000225u32.into(),
            CallFailed(status) => status,
        }
    }
}

macro_rules! WdfCall {
    ($name:ident ( $($args:expr),* )) => {{
        let fn_handle = {
            ::paste::paste! {
                const FN_INDEX: usize = ::wdf_umdf_sys::WDFFUNCENUM::[<$name TableIndex>].0 as usize;

                // validate that wdf function can be used
                let is_available = ::wdf_umdf_sys::WdfIsFunctionAvailable!($name);

                if is_available {
                    // SAFETY: Only immutable accesses are done to this
                    let fn_table = unsafe { ::wdf_umdf_sys::WdfFunctions_02033 };

                    // SAFETY: Read-only, initialized by the time we use it, and checked to be in bounds
                    let fn_handle = unsafe {
                        fn_table
                        .add(FN_INDEX)
                        .cast::<::wdf_umdf_sys::[<PFN_ $name:upper>]>()
                    };

                    // SAFETY: Ensured that this is present by if condition from `WdfIsFunctionAvailable!`
                    let fn_handle = unsafe { fn_handle.read() };
                    // SAFETY: All available function handles are not null
                    let fn_handle = unsafe { fn_handle.unwrap_unchecked() };

                    Ok(fn_handle)
                } else {
                    Err($crate::WdfError::WdfFunctionNotAvailable(concat!(stringify!($name), " is not available")))
                }
            }
        };

        if let Ok(fn_handle) = fn_handle {
            // SAFETY: Pointer to globals is always immutable
            let globals = unsafe { ::wdf_umdf_sys::WdfDriverGlobals };

            // SAFETY: None. User is responsible for safety and must use their own unsafe block
            Ok(fn_handle(globals, $($args),*))
        } else {
            // SAFETY: We checked if it was Ok above, and it clearly isn't
            Err(unsafe {
                fn_handle.unwrap_err_unchecked()
            })
        }
    }};
}

/// Unlike the official WDF_DECLARE_CONTEXT_TYPE macro, you only need to declare this on the actual data struct you'll be using
/// Safety is maintained through a Mutex of the underlying data.
///
/// This generates a type `WdfObject$context_type`, you can access associated fns on it for init/get/drop
#[macro_export]
macro_rules! WDF_DECLARE_CONTEXT_TYPE {
    ($sv:vis $context_type:ident) => {
        $crate::paste! {
            #[allow(non_upper_case_globals)]
            $sv static [<WDF_ $context_type _TYPE_INFO>]: $crate::once_cell::sync::Lazy<$crate::wdf_umdf_sys::_WDF_OBJECT_CONTEXT_TYPE_INFO> =
                $crate::once_cell::sync::Lazy::new(|| $crate::wdf_umdf_sys::_WDF_OBJECT_CONTEXT_TYPE_INFO {
                    Size: ::std::mem::size_of::<$crate::wdf_umdf_sys::_WDF_OBJECT_CONTEXT_TYPE_INFO>() as u32,
                    ContextName: concat!("WdfObject", stringify!($context_type), "\0").as_ptr().cast::<::std::ffi::c_char>(),
                    ContextSize: ::std::mem::size_of::<[<WdfObject $context_type>]>(),
                    UniqueType: &*[<WDF_ $context_type _TYPE_INFO>],
                    EvtDriverGetUniqueContextType: None,
                });

            #[repr(transparent)]
            $sv struct [<WdfObject $context_type>](Option<Box<::std::sync::Mutex<$context_type>>>);

            impl [<WdfObject $context_type>] {
                /// SAFETY:
                /// - Must not call if this type is already in use
                /// - No other mutable/non-mutable refs can exist to type when this is called, or it will alias
                ///
                /// This will overwrite data contained for this type. DO NOT use if this type has already been initialized
                $sv unsafe fn init(handle: $crate::wdf_umdf_sys::WDFOBJECT, value: $context_type) -> Result<(), $crate::WdfError> {
                    let context = unsafe {
                        $crate::WdfObjectGetTypedContextWorker(
                            handle,
                            &*[<WDF_ $context_type _TYPE_INFO>]
                        )?
                    } as *mut Self;

                    let mut_ref = &mut *context;
                    // Set to none so it can drop
                    mut_ref.0 = Some(Box::new(::std::sync::Mutex::new(value)));

                    Ok(())
                }

                /// SAFETY:
                /// - Must not drop if it's still in use elsewhere
                /// - No other mutable/non-mutable refs can exist to data when this is called, or it will alias
                ///
                /// This will overwrite data contained for this type
                $sv unsafe fn drop(handle: $crate::wdf_umdf_sys::WDFOBJECT) -> Result<(), $crate::WdfError> {
                    let context = $crate::WdfObjectGetTypedContextWorker(
                            handle,
                            &*[<WDF_ $context_type _TYPE_INFO>]
                        )? as *mut Self;

                    let mut_ref = &mut *context;
                    // Set to none so it can drop
                    mut_ref.0 = None;

                    Ok(())
                }

                /// Get the context from the wdfobject
                $sv fn get<'a>(handle: $crate::wdf_umdf_sys::WDFOBJECT) -> Result<&'a Option<Box<::std::sync::Mutex<$context_type>>>, $crate::WdfError> {
                    let context = unsafe {
                        $crate::WdfObjectGetTypedContextWorker(
                            handle,
                            &*[<WDF_ $context_type _TYPE_INFO>]
                        )?
                    } as *mut Self;

                    Ok(&unsafe { &*context }.0)
                }
            }
        }
    };
}

pub unsafe fn WdfDriverCreate(
    // in
    DriverObject: PDRIVER_OBJECT,
    // in
    RegistryPath: PCUNICODE_STRING,
    // in, optional
    DriverAttributes: Option<PWDF_OBJECT_ATTRIBUTES>,
    // in
    DriverConfig: PWDF_DRIVER_CONFIG,
    // out, optional
    Driver: Option<&mut WDFDRIVER>,
) -> Result<NTSTATUS, WdfError> {
    let status = WdfCall! {
        WdfDriverCreate(
            DriverObject,
            RegistryPath,
            DriverAttributes.unwrap_or(WDF_NO_OBJECT_ATTRIBUTES!()),
            DriverConfig,
            Driver
                .map(|d| d as *mut _)
                .unwrap_or(WDF_NO_HANDLE!() as *mut *mut _)
        )
    }?;

    if status.is_success() {
        Ok(status)
    } else {
        Err(WdfError::CallFailed(status))
    }
}

pub unsafe fn WdfDeviceCreate(
    // in, out
    DeviceInit: &mut PWDFDEVICE_INIT,
    // in, optional
    DeviceAttributes: Option<&mut WDF_OBJECT_ATTRIBUTES>,
    // out
    Device: &mut WDFDEVICE,
) -> Result<NTSTATUS, WdfError> {
    let status = WdfCall! {
        WdfDeviceCreate(
            DeviceInit,
            DeviceAttributes.map(|d| d as *mut _).unwrap_or(WDF_NO_OBJECT_ATTRIBUTES!() as *mut _),
            Device
        )
    }?;

    if status.is_success() {
        Ok(status)
    } else {
        Err(WdfError::CallFailed(status))
    }
}

pub unsafe fn WdfDeviceInitSetPnpPowerEventCallbacks(
    // in
    DeviceInit: PWDFDEVICE_INIT,
    // in
    PnpPowerEventCallbacks: *mut _WDF_PNPPOWER_EVENT_CALLBACKS,
) -> Result<(), WdfError> {
    WdfCall! {
        WdfDeviceInitSetPnpPowerEventCallbacks(
            DeviceInit,
            PnpPowerEventCallbacks
        )
    }
}

pub unsafe fn WdfObjectGetTypedContextWorker(
    // in
    Handle: WDFOBJECT,
    // in
    TypeInfo: PCWDF_OBJECT_CONTEXT_TYPE_INFO,
) -> Result<*mut c_void, WdfError> {
    WdfCall! {
        WdfObjectGetTypedContextWorker(
            Handle,
            TypeInfo
        )
    }
}
