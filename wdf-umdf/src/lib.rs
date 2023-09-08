mod iddcx;
mod wdf;

pub use iddcx::*;
pub use once_cell;
pub use paste::paste;
pub use wdf::*;
pub use wdf_umdf_sys;

use wdf_umdf_sys::NTSTATUS;

// Helper to allow unwrapping Result into NTSTATUS
pub trait IntoHelper<E> {
    /// Allow to convert Result into one NTSTATUS
    fn into_status(self) -> NTSTATUS;
    /// Separate NTSTATUS result into success/error variants
    fn into_result(self) -> Result<NTSTATUS, E>;
}

impl<R: Into<NTSTATUS> + Into<E> + Copy, E: Into<NTSTATUS>> IntoHelper<E> for Result<R, E> {
    fn into_status(self) -> NTSTATUS {
        match self {
            Ok(t) => t.into(),
            Err(e) => e.into(),
        }
    }

    fn into_result(self) -> Result<NTSTATUS, E> {
        match self {
            // Ok means the call returned, not that there wasn't an error
            Ok(t) => {
                let status: NTSTATUS = t.into();

                if status.is_success() {
                    Ok(status)
                } else {
                    Err(t.into())
                }
            }

            // However, Err is always an error
            Err(e) => Err(e),
        }
    }
}
