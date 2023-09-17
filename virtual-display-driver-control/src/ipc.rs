use std::io::Write;

use driver_ipc::DriverCommand;

use crate::app::App;

pub fn ipc_call<T: Into<DriverCommand>>(app: &mut App, data: T) {
    let command: DriverCommand = data.into();
    let json = serde_json::to_string(&command).unwrap();
    let len = json.len().to_le_bytes();

    app.connection.write_all(&len).unwrap();
    app.connection.write_all(json.as_bytes()).unwrap();
}
