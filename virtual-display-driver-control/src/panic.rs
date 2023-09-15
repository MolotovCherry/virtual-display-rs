#[cfg(debug_assertions)]
use std::backtrace::Backtrace;
use std::panic;

use crate::popup::{display_popup, MessageBoxIcon};

pub fn set_hook() {
    panic::set_hook(Box::new(|v| {
        let message;

        // debug mode, get full backtrace
        #[cfg(debug_assertions)]
        {
            let backtrace = Backtrace::force_capture();
            message = format!("{v}\n\nstack backtrace:\n{backtrace}");
        }

        // otherwise just print the panic since we don't have a backtrace
        #[cfg(not(debug_assertions))]
        {
            message = v.to_string();
        }

        display_popup("Oh no :(", &message, MessageBoxIcon::Error);
    }));
}
