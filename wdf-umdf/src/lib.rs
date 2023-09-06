mod iddcx;
mod wdf;

pub use iddcx::*;
pub use once_cell;
pub use paste::paste;
pub use wdf::*;
pub use wdf_umdf_sys;

use wdf_umdf_sys::NTSTATUS;

// Helper to allow unwrapping Result into NTSTATUS
pub trait IntoStatus {
    fn into_status(self) -> NTSTATUS;
}

impl<T: Into<NTSTATUS>, E: Into<NTSTATUS>> IntoStatus for Result<T, E> {
    fn into_status(self) -> NTSTATUS {
        match self {
            Ok(t) => t.into(),
            Err(e) => e.into(),
        }
    }
}
