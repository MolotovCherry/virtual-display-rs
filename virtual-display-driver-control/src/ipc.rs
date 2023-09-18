use std::io::Write;

use driver_ipc::DriverCommand;

use crate::app::TcpWrapper;

pub fn ipc_call<T: Into<DriverCommand>>(con: &mut TcpWrapper, data: T) {
    let command: DriverCommand = data.into();
    let json = serde_json::to_string(&command).unwrap();
    let len = json.len().to_le_bytes();

    con.write_all(&len).unwrap();
    con.write_all(json.as_bytes()).unwrap();
}
