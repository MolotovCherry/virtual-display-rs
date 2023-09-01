pub use paste::paste;
pub use static_assertions;
pub use wdf_umdf_sys;

use wdf_umdf_sys::{WdfDriverGlobals, WdfFunctions_02033, _WDF_DRIVER_GLOBALS};

#[macro_export]
macro_rules! WdfFunction {
    ($name:ident ( $($args:expr),* $(,)? )) => {{
        let fn_handle = {
            $crate::paste! {
                const FN_INDEX: usize = $crate::wdf_umdf_sys::WDFFUNCENUM::[<$name TableIndex>].0 as usize;

                // assert to always be an available index
                $crate::static_assertions::const_assert!(FN_INDEX < wdf_umdf_sys::WDF_ALWAYS_AVAILABLE_FUNCTION_COUNT as usize);

                // SAFETY: Only immutable accesses are done to this
                let fn_table = unsafe { WdfFunctions_02033 };

                // SAFETY: Read-only, initialized by the time we use it, and checked to be in bounds
                let fn_handle = unsafe {
                    fn_table,
                    .add(FN_INDEX)
                    .cast::<$crate::wdf_umdf_sys::[<PFN_ $name:upper>]>()
                };

                // SAFETY: Ensured that this is present by the static assert
                let fn_handle = unsafe { fn_handle.read() };
                // SAFETY: All available function handles are not null
                let fn_handle = unsafe { fn_handle.unwrap_unchecked() };

                fn_handle
            }
        };

        // SAFETY: Pointer to globals is always immutable
        let globals = unsafe { $crate::wdf_umdf_sys::WdfDriverGlobals };

        // SAFETY: None
        //         User is responsible for safety and must write their own unsafe block
        fn_handle(globals, $($args),*)
    }};
}
