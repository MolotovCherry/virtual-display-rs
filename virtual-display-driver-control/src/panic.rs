use std::panic;

#[cfg(debug_assertions)]
use crate::backtrace::CaptureBacktrace;
use crate::popup::{display_popup, MessageBoxIcon};

pub fn set_hook() {
    panic::set_hook(Box::new(|v| {
        let message = v.to_string();

        // debug mode, get full backtrace
        #[cfg(debug_assertions)]
        {
            let backtrace = CaptureBacktrace.to_string();

            let message = format!("{v}\n\nstack backtrace:\n{backtrace}");
            eprintln!("{message}");
        }

        display_popup("Oh no :(", &message, MessageBoxIcon::Error);
    }));
}
