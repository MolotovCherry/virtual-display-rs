#[cfg(target_os = "windows")]
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::UI::{
        Input::KeyboardAndMouse::GetActiveWindow,
        WindowsAndMessaging::{
            MessageBoxW, MB_ICONERROR, MB_ICONINFORMATION, MB_TASKMODAL, MESSAGEBOX_STYLE,
        },
    },
};

pub enum MessageBoxIcon {
    Information,
    Error,
}

impl From<MessageBoxIcon> for MESSAGEBOX_STYLE {
    fn from(value: MessageBoxIcon) -> Self {
        match value {
            MessageBoxIcon::Information => MB_ICONINFORMATION,
            MessageBoxIcon::Error => MB_ICONERROR,
        }
    }
}

pub fn display_popup(title: &str, message: &str, icon: MessageBoxIcon) {
    // these must be explicitly assigned, otherwise they will be temporary and drop
    // and create an invalid pointer, causing corruption and UB
    let h_title = HSTRING::from(title);
    let h_message = HSTRING::from(message);

    let title = PCWSTR::from_raw(h_title.as_ptr());
    let message = PCWSTR::from_raw(h_message.as_ptr());

    let mut icon = icon.into();

    unsafe {
        let hwnd = {
            let _hwnd = GetActiveWindow();
            if _hwnd.0 == 0 {
                None
            } else {
                icon |= MB_TASKMODAL;
                Some(_hwnd)
            }
        };

        if let Some(hwnd) = hwnd {
            MessageBoxW(hwnd, message, title, icon);
        } else {
            MessageBoxW(None, message, title, icon);
        }
    }
}
