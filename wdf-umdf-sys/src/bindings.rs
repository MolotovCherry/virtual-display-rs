// stand-in type replacing NTSTATUS in the bindings
use crate::NTSTATUS;

include!(concat!(env!("OUT_DIR"), "/umdf.rs"));

// fails to build without this symbol
#[no_mangle]
pub static IddMinimumVersionRequired: ULONG = 6;
