#![allow(non_snake_case)]

use wdf_umdf_sys::{IDD_CX_CLIENT_CONFIG, NTSTATUS, WDFDEVICE_INIT};

#[derive(Debug, thiserror::Error)]
pub enum IddCxError {
    #[error("{0}")]
    IddCxFunctionNotAvailable(&'static str),
}

impl From<IddCxError> for NTSTATUS {
    fn from(value: IddCxError) -> Self {
        use IddCxError::*;
        match value {
            IddCxFunctionNotAvailable(_) => 0xC0000225u32.into(),
        }
    }
}

macro_rules! IddCxCall {
    ($name:ident ( $($args:expr),* )) => {{
        let fn_handle = {
            ::paste::paste! {
                const FN_INDEX: usize = ::wdf_umdf_sys::IDDFUNCENUM::[<$name TableIndex>].0 as usize;

                // validate that wdf function can be used
                let is_available = ::wdf_umdf_sys::IddCxIsFunctionAvailable!($name);

                if is_available {
                    // SAFETY: Only immutable accesses are done to this
                    let fn_table = unsafe { ::wdf_umdf_sys::IddFunctions };

                    // SAFETY: Ensured that this is present by if condition from `WdfIsFunctionAvailable!`
                    let fn_handle = unsafe {
                        (fn_table.get_unchecked(FN_INDEX) as *const Option<_>)
                            .cast::<::wdf_umdf_sys::[<PFN_ $name:upper>]>()
                    };

                    // SAFETY: Ensured that this is present by if condition from `IddIsFunctionAvailable!`
                    let fn_handle = unsafe { fn_handle.read() };
                    // SAFETY: All available function handles are not null
                    let fn_handle = unsafe { fn_handle.unwrap_unchecked() };

                    Ok(fn_handle)
                } else {
                    Err($crate::IddCxError::IddCxFunctionNotAvailable(concat!(stringify!($name), " is not available")))
                }
            }
        };

        if let Ok(fn_handle) = fn_handle {
            // SAFETY: Pointer to globals is always immutable
            let globals = unsafe { ::wdf_umdf_sys::IddDriverGlobals };

            // SAFETY: None. User is responsible for safety and must use their own unsafe block
            // specify NTSTATUS type so unit can also convert into
            Ok(fn_handle(globals, $($args),*))
        } else {
            // SAFETY: We checked if it was Ok above, and it clearly isn't
            Err(unsafe {
                fn_handle.unwrap_err_unchecked()
            })
        }
    }};
}

pub unsafe fn IddCxDeviceInitConfig(
    // in, out
    DeviceInit: &mut WDFDEVICE_INIT,
    // in
    Config: &IDD_CX_CLIENT_CONFIG,
) -> Result<NTSTATUS, IddCxError> {
    IddCxCall! {
        IddCxDeviceInitConfig(
            DeviceInit,
            Config
        )
    }
}
