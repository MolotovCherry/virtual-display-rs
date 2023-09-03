#[cfg(debug_assertions)]
use std::backtrace::Backtrace;
use std::panic;

use log::error;

pub fn set_hook() {
    panic::set_hook(Box::new(|v| {
        // debug mode, get full backtrace
        #[cfg(debug_assertions)]
        {
            let backtrace = Backtrace::force_capture();
            error!("{v}\n\nstack backtrace:\n{backtrace}");
        }

        // otherwise just print the panic since we don't have a backtrace
        #[cfg(not(debug_assertions))]
        error!("{v}");
    }));
}
