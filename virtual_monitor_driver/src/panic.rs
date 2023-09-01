#[cfg(debug_assertions)]
use std::backtrace::Backtrace;
use std::panic;

use crate::popup::{display_popup, MessageBoxIcon};

// TODO: Parse message for debug mode using
// __rust_end_short_backtrace
// __rust_begin_short_backtrace
pub fn set_hook() {
    panic::set_hook(Box::new(|v| {
        #[cfg(debug_assertions)]
        {
            let panic_msg = v.to_string();
            let backtrace = Backtrace::force_capture();

            let full_backtrace = backtrace.to_string();

            eprintln!("{}\n\nstack backtrace:\n{}", panic_msg, full_backtrace);
        }

        display_popup(
            "VirtualDisplayDriver panicked :(",
            &v.to_string(),
            MessageBoxIcon::Error,
        );
    }));
}
