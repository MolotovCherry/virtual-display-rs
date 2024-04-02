use std::io::Write as _;

use eyre::Context as _;
use serde::{de::DeserializeOwned, Serialize};
use win_pipes::{NamedPipeClientReader, NamedPipeClientWriter};

use crate::{ClientCommand, DriverCommand, Id, Monitor, RequestCommand};

/// A thin api client over the driver api with all the essential api.
/// Does not track state for you
///
/// This is cloneable and won't drop underlying handle until all instances
/// are dropped
#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) writer: NamedPipeClientWriter,
    pub(crate) reader: NamedPipeClientReader,
}

impl Client {
    pub fn connect() -> eyre::Result<Self> {
        let (reader, writer) =
            win_pipes::NamedPipeClientOptions::new("virtualdisplaydriver")
                .wait()
                .access_duplex()
                .mode_message()
                .create()
                .context("Failed to connect to Virtual Display Driver; please ensure the driver is installed and working")?;

        Ok(Self { reader, writer })
    }

    /// Notifies driver of changes (additions/updates/removals)
    pub fn notify(&mut self, monitors: &[Monitor]) -> eyre::Result<()> {
        let command = DriverCommand::Notify(monitors.to_owned());

        send_command(&mut self.writer, &command)
    }

    /// Remove specific monitors by id
    pub fn remove(&mut self, ids: &[Id]) -> eyre::Result<()> {
        let command = DriverCommand::Remove(ids.to_owned());

        send_command(&mut self.writer, &command)
    }

    /// Remove all monitors
    pub fn remove_all(&mut self) -> eyre::Result<()> {
        let command = DriverCommand::RemoveAll;

        send_command(&mut self.writer, &command)
    }

    /// Receive generic reply
    ///
    /// This is required because a reply could be any of these at any moment
    pub fn receive(&mut self) -> eyre::Result<ClientCommand> {
        receive_command(&mut self.reader)
    }

    /// Request state update
    /// use `receive()` to get the reply
    pub fn request_state(&mut self) -> eyre::Result<()> {
        let command = RequestCommand::State;

        send_command(&mut self.writer, &command)
    }
}

fn send_command(
    ipc_writer: &mut NamedPipeClientWriter,
    command: &impl Serialize,
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

fn receive_command<T: DeserializeOwned>(ipc_reader: &mut NamedPipeClientReader) -> eyre::Result<T> {
    let response = ipc_reader
        .read_full()
        .wrap_err("failed to read from driver pipe")?;
    let command = serde_json::from_slice(&response).wrap_err("failed to deserialize command")?;

    Ok(command)
}
