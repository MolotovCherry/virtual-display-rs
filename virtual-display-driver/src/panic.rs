#[cfg(debug_assertions)]
use std::backtrace::Backtrace;
use std::panic;

use log::error;

pub fn set_hook() {
    panic::set_hook(Box::new(|v| {
        // debug mode, get full backtrace
        if cfg!(debug_assertions) {
            let backtrace = Backtrace::force_capture();
            error!("{v}\n\nstack backtrace:\n{backtrace}");
        } else {
            // otherwise just print the panic since we don't have a backtrace
            error!("{v}");
        }
    }));
}
