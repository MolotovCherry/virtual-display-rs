use windows::{
    core::{HSTRING, PCWSTR},
    Win32::UI::WindowsAndMessaging::{
        MessageBoxW, MB_ICONERROR, MB_ICONINFORMATION, MB_ICONWARNING, MESSAGEBOX_STYLE,
    },
};

pub enum MessageBoxIcon {
    Information,
    Warning,
    Error,
}

impl From<MessageBoxIcon> for MESSAGEBOX_STYLE {
    fn from(value: MessageBoxIcon) -> Self {
        match value {
            MessageBoxIcon::Information => MB_ICONINFORMATION,
            MessageBoxIcon::Warning => MB_ICONWARNING,
            MessageBoxIcon::Error => MB_ICONERROR,
        }
    }
}

pub fn display_popup(title: &str, message: &str, icon: MessageBoxIcon) {
    // these are separate because if you do it inline, you'll get a temporary borrow, then drop,
    // then pcwstr will be dangling, and then use-after-free!
    let h_title = HSTRING::from(title);
    let h_message = HSTRING::from(message);

    // these must never outlive their source hstring
    let title = PCWSTR(h_title.as_ptr());
    let message = PCWSTR(h_title.as_ptr());

    let mut icon = icon.into();

    unsafe {
        MessageBoxW(None, message, title, icon);
    }
}
