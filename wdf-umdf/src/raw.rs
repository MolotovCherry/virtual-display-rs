#![allow(non_snake_case)]

use wdf_umdf_sys::{
    NTSTATUS, PCUNICODE_STRING, PDRIVER_OBJECT, PWDF_DRIVER_CONFIG, PWDF_OBJECT_ATTRIBUTES,
    WDFDRIVER, WDF_NO_HANDLE, WDF_NO_OBJECT_ATTRIBUTES,
};

#[derive(Debug, thiserror::Error)]
pub enum WdfError {
    #[error("{0}")]
    WdfFunctionNotAvailable(&'static str),
}

#[macro_export]
macro_rules! WdfCall {
    ($name:ident ( $($args:expr),* )) => {{
        let fn_handle = {
            $crate::paste! {
                const FN_INDEX: usize = $crate::wdf_umdf_sys::WDFFUNCENUM::[<$name TableIndex>].0 as usize;

                // validate that wdf function can be used
                let is_available = $crate::wdf_umdf_sys::WdfIsFunctionAvailable!($name);

                if is_available {
                    // SAFETY: Only immutable accesses are done to this
                    let fn_table = unsafe { $crate::wdf_umdf_sys::WdfFunctions_02033 };

                    // SAFETY: Read-only, initialized by the time we use it, and checked to be in bounds
                    let fn_handle = unsafe {
                        fn_table
                        .add(FN_INDEX)
                        .cast::<$crate::wdf_umdf_sys::[<PFN_ $name:upper>]>()
                    };

                    // SAFETY: Ensured that this is present by the static assert
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
            let globals = unsafe { $crate::wdf_umdf_sys::WdfDriverGlobals };

            // SAFETY: None. User is responsible for safety and must use their own unsafe block
            // specify NTSTATUS type so unit can also convert into
            Ok::<$crate::wdf_umdf_sys::NTSTATUS, _>(fn_handle(globals, $($args),*).into())
        } else {
            // SAFETY: We checked if it was Ok above, and it clearly isn't
            Err(unsafe {
                fn_handle.unwrap_err_unchecked()
            })
        }
    }};
}

#[must_use]
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
    WdfCall! {
        WdfDriverCreate(
            DriverObject,
            RegistryPath,
            DriverAttributes.unwrap_or(WDF_NO_OBJECT_ATTRIBUTES!()),
            DriverConfig,
            Driver
                .map(|d| d as *mut _)
                .unwrap_or(WDF_NO_HANDLE!() as *mut *mut _)
        )
    }
}
