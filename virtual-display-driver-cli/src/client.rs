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
        // let read = self.receive_command()?;
        // println!("{:?}", read);

        self.send_command(&driver_ipc::Command::RequestState)?;
        let response = self.receive_command()?;

        match response {
            driver_ipc::Command::ReplyState(state) => Ok(state),
            _ => eyre::bail!("received unexpected reply from driver pipe"),
        }
    }

    pub fn add(&mut self, monitor: NewMonitor) -> eyre::Result<driver_ipc::Id> {
        let new_id = match monitor.id {
            Some(id) => id,
            None => self.next_id()?,
        };

        let command = driver_ipc::Command::DriverNotify(vec![Monitor {
            id: new_id,
            name: None,
            enabled: true,
            modes: vec![driver_ipc::Mode {
                width: monitor.width,
                height: monitor.height,
                refresh_rates: monitor.refresh_rates,
            }],
        }]);

        self.send_command(&command)?;

        Ok(new_id)
    }

    fn next_id(&mut self) -> eyre::Result<driver_ipc::Id> {
        let monitors = self.list()?;
        let max_id = monitors.iter().map(|monitor| monitor.id).max();

        match max_id {
            Some(id) => Ok(id + 1),
            None => Ok(0),
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

pub struct NewMonitor {
    pub width: driver_ipc::Dimen,
    pub height: driver_ipc::Dimen,
    pub refresh_rates: Vec<driver_ipc::RefreshRate>,
    pub id: Option<driver_ipc::Id>,
}
