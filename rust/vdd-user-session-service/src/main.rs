mod service;
mod set_privileges;

use clap::Parser;
use std::ffi::OsString;
use windows_service::{
    service::{
        ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceState,
        ServiceType,
    },
    service_manager::{ServiceManager, ServiceManagerAccess},
};

use self::service::start_service;

const SERVICE_NAME: &str = "vdd-user-session-initializer";
const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Install the service
    #[arg(short, long, conflicts_with = "uninstall")]
    install: bool,

    /// Uninstall the service
    #[arg(short, long, conflicts_with = "install")]
    uninstall: bool,
}

fn main() -> Result<(), windows_service::Error> {
    let args = Args::parse();

    if args.install {
        let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
        let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

        let service_binary_path = ::std::env::current_exe().unwrap();

        let service_info = ServiceInfo {
            name: OsString::from(SERVICE_NAME),
            display_name: OsString::from("Virtual Display Driver User Session Initializer Service"),
            service_type: SERVICE_TYPE,
            start_type: ServiceStartType::AutoStart,
            error_control: ServiceErrorControl::Normal,
            executable_path: service_binary_path,
            launch_arguments: vec![],
            dependencies: vec![],
            account_name: None, // run as System
            account_password: None,
        };

        let service =
            service_manager.create_service(&service_info, ServiceAccess::CHANGE_CONFIG)?;
        service.set_description("Watches for log on and log off events and starts/stops virtual monitors based on user persisted data")?;

        return Ok(());
    } else if args.uninstall {
        let manager_access = ServiceManagerAccess::CONNECT;
        let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

        let service_access =
            ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE;
        let service = service_manager.open_service(SERVICE_NAME, service_access)?;

        // The service will be marked for deletion as long as this function call succeeds.
        // However, it will not be deleted from the database until it is stopped and all open handles to it are closed.
        service.delete()?;
        // Our handle to it is not closed yet. So we can still query it.
        if service.query_status()?.current_state != ServiceState::Stopped {
            // If the service cannot be stopped, it will be deleted when the system restarts.
            service.stop()?;
        }

        return Ok(());
    }

    start_service()?;

    Ok(())
}
