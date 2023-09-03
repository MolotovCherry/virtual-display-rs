//
// Code in this project was adapated from IddSampleDriver
// https://github.com/ge9/IddSampleDriver
//

mod direct_3d_device;
mod entry;
mod indirect_device_context;
mod panic;
mod swap_chain_processor;
mod wdf;

use std::ffi::c_void;

use log::Level;
use wdf_umdf_sys::DLL_PROCESS_ATTACH;
use windows::Win32::Foundation::HINSTANCE;
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

const EVENT_NAME: &str = "Virtual Display Driver";

#[no_mangle]
#[allow(unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _: *mut c_void) -> bool {
    let mut successful = true;

    match call_reason {
        DLL_PROCESS_ATTACH => 'exit: {
            // register event log
            if !is_event_registered(EVENT_NAME) {
                successful = eventlog::register(EVENT_NAME).is_ok();

                if !successful {
                    break 'exit;
                }
            }

            successful = eventlog::init(
                EVENT_NAME,
                if cfg!(debug_assertions) {
                    Level::Trace
                } else {
                    Level::Info
                },
            )
            .is_ok();

            if !successful {
                break 'exit;
            }

            panic::set_hook();
        }

        _ => (),
    }

    successful
}

fn is_event_registered(name: &str) -> bool {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = format!(
        r"SYSTEM\CurrentControlSet\Services\EventLog\Application\{}",
        name
    );

    // try to open key
    hklm.open_subkey(key).map(|_| true).unwrap_or(false)
}
