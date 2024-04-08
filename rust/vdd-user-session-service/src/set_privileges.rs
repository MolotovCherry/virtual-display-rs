use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{CloseHandle, HANDLE, LUID},
        Security::{
            AdjustTokenPrivileges, LookupPrivilegeValueW, SE_PRIVILEGE_ENABLED,
            TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_PRIVILEGES_ATTRIBUTES,
        },
        System::Threading::{OpenProcess, OpenProcessToken, PROCESS_QUERY_INFORMATION},
    },
};

pub fn set_privilege(name: PCWSTR, state: bool) -> bool {
    let Ok(handle) = (unsafe { OpenProcess(PROCESS_QUERY_INFORMATION, false, std::process::id()) })
    else {
        return false;
    };

    let mut token_handle = HANDLE::default();
    if unsafe { OpenProcessToken(handle, TOKEN_ADJUST_PRIVILEGES, &mut token_handle).is_err() } {
        return false;
    }

    let mut luid = LUID::default();

    if unsafe { LookupPrivilegeValueW(PCWSTR::null(), name, &mut luid).is_err() } {
        return false;
    }

    let mut tp = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        ..Default::default()
    };

    tp.Privileges[0].Luid = luid;

    if state {
        tp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
    } else {
        tp.Privileges[0].Attributes = TOKEN_PRIVILEGES_ATTRIBUTES(0u32);
    }

    if unsafe {
        #[allow(clippy::cast_possible_truncation)]
        AdjustTokenPrivileges(
            token_handle,
            false,
            Some(&tp),
            std::mem::size_of::<TOKEN_PRIVILEGES>() as u32,
            None,
            None,
        )
        .is_err()
    } {
        return false;
    }

    if unsafe { CloseHandle(handle).is_err() } {
        return false;
    }

    if unsafe { CloseHandle(token_handle).is_err() } {
        return false;
    }

    true
}
