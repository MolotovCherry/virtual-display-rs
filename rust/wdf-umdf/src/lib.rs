mod iddcx;
mod wdf;

use std::any::Any;

pub use paste::paste;

pub use iddcx::*;
pub use wdf::*;
pub use wdf_umdf_sys;

use wdf_umdf_sys::NTSTATUS;

/// Used for the macros so they can correctly convert a functions result
fn is_nt_error(val: &dyn Any, other_is_error: bool) -> bool {
    if let Some(status) = val.downcast_ref::<NTSTATUS>() {
        return !status.is_success();
    }

    // other errors which may not be error codes, but may also be
    // such as HRESULT == i32
    if other_is_error {
        if let Some(status) = val.downcast_ref::<i32>() {
            let status = NTSTATUS(*status);
            return !status.is_success();
        }
    }

    false
}
