use std::io::Write as _;

use driver_ipc::Monitor;
use eyre::Context;
use win_pipes::{NamedPipeClientReader, NamedPipeClientWriter};

pub struct Client {
    reader: NamedPipeClientReader,
    writer: NamedPipeClientWriter,
}

impl Client {
    pub fn connect() -> eyre::Result<Self> {
        let (reader, writer) = win_pipes::NamedPipeClientOptions::new("virtualdisplaydriver")
            .wait()
            .access_duplex()
            .mode_message()
            .create()?;

        Ok(Self { reader, writer })
    }

    pub fn list(&mut self) -> eyre::Result<Vec<Monitor>> {
        self.send_command(&driver_ipc::Command::RequestState)?;
        let response = self.receive_command()?;

        match response {
            driver_ipc::Command::ReplyState(state) => Ok(state),
            _ => eyre::bail!("received unexpected reply from driver pipe"),
        }
    }

    pub fn get(&mut self, id: driver_ipc::Id) -> eyre::Result<Monitor> {
        let monitors = self.list()?;
        let monitor = monitors.into_iter().find(|monitor| monitor.id == id);

        match monitor {
            Some(monitor) => Ok(monitor),
            None => eyre::bail!("no virtual monitor with ID {} found", id),
        }
    }

    pub fn notify(&mut self, monitors: Vec<driver_ipc::Monitor>) -> eyre::Result<()> {
        let command = driver_ipc::Command::DriverNotify(monitors);

        self.send_command(&command)?;

        Ok(())
    }

    pub fn remove(&mut self, ids: Vec<driver_ipc::Id>) -> eyre::Result<()> {
        let command = driver_ipc::Command::DriverRemove(ids);

        self.send_command(&command)?;

        Ok(())
    }

    pub fn remove_all(&mut self) -> eyre::Result<()> {
        let command = driver_ipc::Command::DriverRemoveAll;

        self.send_command(&command)?;

        Ok(())
    }

    pub fn new_id(&mut self, preferred_id: Option<driver_ipc::Id>) -> eyre::Result<driver_ipc::Id> {
        let monitors = self.list()?;

        if let Some(id) = preferred_id {
            if monitors.iter().any(|monitor| monitor.id == id) {
                eyre::bail!("monitor with ID {} already exists", id);
            }

            Ok(id)
        } else {
            let max_id = monitors.iter().map(|monitor| monitor.id).max();
            let id = match max_id {
                Some(id) => id + 1,
                None => 0,
            };
            Ok(id)
        }
    }

    fn send_command(&mut self, command: &driver_ipc::Command) -> eyre::Result<()> {
        // Create a vector with the full message, then send it as a single
        // write. This is required because the pipe is in message mode.
        let message = serde_json::to_vec(command).wrap_err("failed to serialize command")?;
        self.writer
            .write_all(&message)
            .wrap_err("failed to write to driver pipe")?;
        self.writer
            .flush()
            .wrap_err("failed to flush driver pipe")?;

        Ok(())
    }

    fn receive_command(&mut self) -> eyre::Result<driver_ipc::Command> {
        let response = self
            .reader
            .read_full()
            .wrap_err("failed to read from driver pipe")?;
        let command =
            serde_json::from_slice(&response).wrap_err("failed to deserialize command")?;

        Ok(command)
    }
}
