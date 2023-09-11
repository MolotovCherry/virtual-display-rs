#![allow(clippy::all)]

// stand-in type replacing NTSTATUS in the bindings
use crate::NTSTATUS;

include!(concat!(env!("OUT_DIR"), "/umdf.rs"));

// required for some macros
unsafe impl Send for _WDF_OBJECT_CONTEXT_TYPE_INFO {}
unsafe impl Sync for _WDF_OBJECT_CONTEXT_TYPE_INFO {}

// fails to build without this symbol
#[no_mangle]
pub static IddMinimumVersionRequired: ULONG = 6;
