use std::panic;

use crate::popup::{display_popup, MessageBoxIcon};

pub fn set_hook() {
    panic::set_hook(Box::new(|v| {
        let message = v.to_string();

        display_popup("Oh no :(", &message, MessageBoxIcon::Error);
    }));
}
