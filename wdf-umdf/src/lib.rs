mod iddcx;
mod wdf;

use std::any::Any;

pub use paste::paste;

pub use iddcx::*;
pub use wdf::*;
pub use wdf_umdf_sys;

use wdf_umdf_sys::NTSTATUS;

/// Used for the macros so they can correctly convert a functions result
fn is_nt_error(val: &dyn Any) -> bool {
    if let Some(status) = val.downcast_ref::<NTSTATUS>() {
        !status.is_success()
    } else if let Some(status) = val.downcast_ref::<i32>() {
        let status = NTSTATUS(*status);
        !status.is_success()
    } else {
        false
    }
}
