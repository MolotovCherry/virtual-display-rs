use std::{collections::HashSet, io::Write as _};

use driver_ipc::Monitor;
use eyre::Context;
use win_pipes::{NamedPipeClientReader, NamedPipeClientWriter};

pub struct Client {
    writer: NamedPipeClientWriter,
    state: Vec<Monitor>,
}

impl Client {
    pub fn connect() -> eyre::Result<Self> {
        let (mut reader, mut writer) =
            win_pipes::NamedPipeClientOptions::new("virtualdisplaydriver")
                .wait()
                .access_duplex()
                .mode_message()
                .create()?;

        send_command(&mut writer, &driver_ipc::Command::RequestState)?;
        let state = receive_command(&mut reader)?;
        let driver_ipc::Command::ReplyState(state) = state else {
            eyre::bail!("received unexpected reply from driver pipe");
        };

        Ok(Self { writer, state })
    }

    pub fn monitors(&self) -> &[Monitor] {
        &self.state
    }

    pub fn get(&mut self, id: driver_ipc::Id) -> eyre::Result<Monitor> {
        let monitor = self.state.iter().find(|monitor| monitor.id == id);

        match monitor {
            Some(monitor) => Ok(monitor.clone()),
            None => eyre::bail!("no virtual monitor with ID {} found", id),
        }
    }

    pub fn notify(&mut self, monitors: Vec<driver_ipc::Monitor>) -> eyre::Result<()> {
        let command = driver_ipc::Command::DriverNotify(monitors);

        send_command(&mut self.writer, &command)?;

        Ok(())
    }

    pub fn remove(&mut self, ids: Vec<driver_ipc::Id>) -> eyre::Result<()> {
        let command = driver_ipc::Command::DriverRemove(ids);

        send_command(&mut self.writer, &command)?;

        Ok(())
    }

    pub fn remove_all(&mut self) -> eyre::Result<()> {
        let command = driver_ipc::Command::DriverRemoveAll;

        send_command(&mut self.writer, &command)?;

        Ok(())
    }

    pub fn new_id(&mut self, preferred_id: Option<driver_ipc::Id>) -> eyre::Result<driver_ipc::Id> {
        let existing_ids = self
            .state
            .iter()
            .map(|monitor| monitor.id)
            .collect::<HashSet<_>>();

        if let Some(id) = preferred_id {
            eyre::ensure!(
                !existing_ids.contains(&id),
                "monitor with ID {id} already exists"
            );

            Ok(id)
        } else {
            #[allow(clippy::maybe_infinite_iter)]
            let new_id = (0..)
                .find(|id| !existing_ids.contains(id))
                .expect("failed to get a new ID");
            Ok(new_id)
        }
    }
}

fn send_command(
    ipc_writer: &mut NamedPipeClientWriter,
    command: &driver_ipc::Command,
) -> eyre::Result<()> {
    // Create a vector with the full message, then send it as a single
    // write. This is required because the pipe is in message mode.
    let message = serde_json::to_vec(command).wrap_err("failed to serialize command")?;
    ipc_writer
        .write_all(&message)
        .wrap_err("failed to write to driver pipe")?;
    ipc_writer.flush().wrap_err("failed to flush driver pipe")?;

    Ok(())
}

fn receive_command(ipc_reader: &mut NamedPipeClientReader) -> eyre::Result<driver_ipc::Command> {
    let response = ipc_reader
        .read_full()
        .wrap_err("failed to read from driver pipe")?;
    let command = serde_json::from_slice(&response).wrap_err("failed to deserialize command")?;

    Ok(command)
}
