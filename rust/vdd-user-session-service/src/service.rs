use std::{ffi::OsString, io::ErrorKind, sync::mpsc, time::Duration};

use driver_ipc::{Client, Monitor};
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    Security::{ImpersonateLoggedOnUser, SE_TCB_NAME},
    System::RemoteDesktop::WTSQueryUserToken,
};
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        SessionChangeReason,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher,
};
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_READ},
    RegKey,
};

use crate::{set_privileges::set_privilege, SERVICE_NAME, SERVICE_TYPE};

define_windows_service!(ffi_service_main, service_main);

pub fn start_service() -> windows_service::Result<()> {
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;
    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
fn service_main(arguments: Vec<OsString>) {
    if let Err(_e) = run_service(&arguments) {
        // error handling
    }
}

#[allow(clippy::too_many_lines)]
fn run_service(_arguments: &[OsString]) -> windows_service::Result<()> {
    // escalate privileges so we can get the logged on user token
    if !set_privilege(SE_TCB_NAME, true) {
        let io = std::io::Error::new(ErrorKind::Other, "Failed to grant SE_TCB_NAME");
        return Err(windows_service::Error::Winapi(io));
    }

    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    let mut latest_session = 0;

    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop => {
                shutdown_tx.send(()).unwrap();
                ServiceControlHandlerResult::NoError
            }

            ServiceControl::SessionChange(param) => {
                match param.reason {
                    SessionChangeReason::SessionLogon
                    | SessionChangeReason::RemoteConnect
                    | SessionChangeReason::SessionUnlock => {
                        // skip if this was already ran for a particular session
                        if latest_session == param.notification.session_id {
                            return ServiceControlHandlerResult::NoError;
                        }

                        latest_session = param.notification.session_id;

                        let mut token = HANDLE::default();
                        if unsafe {
                            WTSQueryUserToken(param.notification.session_id, &mut token).is_err()
                        } {
                            return ServiceControlHandlerResult::Other(0x1);
                        }

                        // impersonate user for current user reg call
                        if unsafe { ImpersonateLoggedOnUser(token).is_err() } {
                            return ServiceControlHandlerResult::Other(0x2);
                        }

                        let hklm = RegKey::predef(HKEY_CURRENT_USER);
                        let key = r"SOFTWARE\VirtualDisplayDriver";

                        let Ok(driver_settings) = hklm.open_subkey_with_flags(key, KEY_READ) else {
                            return ServiceControlHandlerResult::NoError;
                        };

                        let monitors = driver_settings
                            .get_value::<String, _>("data")
                            .map(|data| {
                                serde_json::from_str::<Vec<Monitor>>(&data).unwrap_or_default()
                            })
                            .unwrap_or_default();

                        let Ok(mut client) = Client::connect() else {
                            return ServiceControlHandlerResult::Other(0x3);
                        };

                        _ = client.notify(monitors);

                        _ = unsafe { CloseHandle(token) };
                    }

                    SessionChangeReason::SessionLogoff => {
                        let Ok(mut client) = Client::connect() else {
                            return ServiceControlHandlerResult::Other(0x3);
                        };

                        _ = client.remove_all();
                    }

                    _ => (),
                }

                ServiceControlHandlerResult::NoError
            }

            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,

            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    // service running
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP | ServiceControlAccept::SESSION_CHANGE,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    // blocking wait for shutdown signal
    _ = shutdown_rx.recv();

    // service stopped
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    Ok(())
}
