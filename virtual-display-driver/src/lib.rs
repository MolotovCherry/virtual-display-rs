//
// Code in this project was adapated from IddSampleDriver
// https://github.com/ge9/IddSampleDriver
//

mod direct_3d_device;
mod entry;
mod indirect_device_context;
mod panic;
mod popup;
mod swap_chain_processor;
mod wdf;

use std::ffi::c_void;

use windows::Win32::Foundation::HINSTANCE;

#[no_mangle]
#[allow(unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _: *mut c_void) -> bool {
    panic::set_hook();

    true
}
