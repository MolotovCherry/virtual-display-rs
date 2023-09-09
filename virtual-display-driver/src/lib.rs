mod callbacks;
mod device_context;
mod direct_3d_device;
mod entry;
mod panic;
mod swap_chain_processor;
mod wdf;

use wdf_umdf_sys::{NTSTATUS, PUNICODE_STRING, PVOID};

//
// This exports the framework entry point function.
// This is the first thing called when the driver loads.
// After it finishes, it calls the exported,
//     DriverEntry: DRIVER_INITIALIZE
// which is defined in `entry.rs`
//
#[link(
    name = "WdfDriverStubUm",
    kind = "static",
    modifiers = "+whole-archive"
)]
extern "C" {
    pub fn FxDriverEntryUm(
        LoaderInterface: PVOID,
        Context: PVOID,
        DriverObject: PVOID,
        RegistryPath: PUNICODE_STRING,
    ) -> NTSTATUS;
}
