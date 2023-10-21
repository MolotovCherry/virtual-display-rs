use std::io::Write;

use driver_ipc::DriverCommand;

use crate::app::PipeWrapper;

pub fn ipc_call<T: Into<DriverCommand>>(con: &mut PipeWrapper, data: T) {
    let command: DriverCommand = data.into();
    let json = serde_json::to_string(&command).unwrap();

    con.write_all(json.as_bytes()).unwrap();
}
