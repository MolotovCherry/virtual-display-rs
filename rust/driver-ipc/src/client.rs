use std::io::{prelude::Read, Write as _};

use eyre::{bail, Context as _};
use log::error;
use serde::{de::DeserializeOwned, Serialize};
use win_pipes::{NamedPipeClientReader, NamedPipeClientWriter};
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_WRITE},
    RegKey,
};

use crate::{ClientCommand, DriverCommand, Id, Monitor, RequestCommand};

// EOF byte used to separate messages
const EOF: u8 = 0x4;

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
                .mode_byte()
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

    /// Persist changes to registry for current user
    pub fn persist(monitors: &[Monitor]) -> eyre::Result<()> {
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        let key = r"SOFTWARE\VirtualDisplayDriver";

        let mut reg_key = hklm.open_subkey_with_flags(key, KEY_WRITE);

        // if open failed, try to create key and subkey
        if let Err(e) = reg_key {
            error!("Failed opening {key}: {e:?}");
            reg_key = hklm.create_subkey(key).map(|(key, _)| key);

            if let Err(e) = reg_key {
                error!("Failed creating {key}: {e:?}");
                bail!("Failed to open or create key {key}");
            }
        }

        let reg_key = reg_key.unwrap();

        let Ok(data) = serde_json::to_string(monitors) else {
            bail!("Failed to convert state to json");
        };

        if reg_key.set_value("data", &data).is_err() {
            bail!("Failed to save reg key");
        }

        Ok(())
    }
}

fn send_command(
    ipc_writer: &mut NamedPipeClientWriter,
    command: &impl Serialize,
) -> eyre::Result<()> {
    // Create a vector with the full message, then send it as a single
    // write. This is required because the pipe is in message mode.
    let mut message = serde_json::to_vec(command).wrap_err("failed to serialize command")?;
    message.push(EOF);
    ipc_writer
        .write_all(&message)
        .wrap_err("failed to write to driver pipe")?;
    ipc_writer.flush().wrap_err("failed to flush driver pipe")?;

    Ok(())
}

fn receive_command<T: DeserializeOwned>(ipc_reader: &mut NamedPipeClientReader) -> eyre::Result<T> {
    let mut msg_buf = Vec::with_capacity(4096);
    let mut buf = vec![0; 4096];

    loop {
        let Ok(size) = ipc_reader.read(&mut buf) else {
            break;
        };

        msg_buf.extend_from_slice(&buf[..size]);

        if msg_buf.last().is_some_and(|&byte| byte == EOF) {
            break;
        }
    }

    // in the following we assume we always get a fully formed message, e.g. no multiple messages

    // pop off EOF
    msg_buf.pop();

    // interior EOF is not valid
    assert!(
        !msg_buf.contains(&EOF),
        "interior eof detected, msg: {msg_buf:?}"
    );

    let command = serde_json::from_slice(&msg_buf).wrap_err("failed to deserialize command")?;

    Ok(command)
}
