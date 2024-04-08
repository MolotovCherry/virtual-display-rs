use std::io::{prelude::Read, Write as _};

use log::error;
use serde::{de::DeserializeOwned, Serialize};
use win_pipes::{NamedPipeClientReader, NamedPipeClientWriter};
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_WRITE},
    RegKey,
};

use crate::{ClientCommand, DriverCommand, Id, Monitor, RequestCommand, Result};

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
    pub fn connect() -> Result<Self> {
        let (reader, writer) = win_pipes::NamedPipeClientOptions::new("virtualdisplaydriver")
            .wait()
            .access_duplex()
            .mode_byte()
            .create()?;

        Ok(Self { reader, writer })
    }

    /// Notifies driver of changes (additions/updates/removals)
    pub fn notify(&mut self, monitors: &[Monitor]) -> Result<()> {
        let command = DriverCommand::Notify(monitors.to_owned());

        send_command(&mut self.writer, &command)
    }

    /// Remove specific monitors by id
    pub fn remove(&mut self, ids: &[Id]) -> Result<()> {
        let command = DriverCommand::Remove(ids.to_owned());

        send_command(&mut self.writer, &command)
    }

    /// Remove all monitors
    pub fn remove_all(&mut self) -> Result<()> {
        let command = DriverCommand::RemoveAll;

        send_command(&mut self.writer, &command)
    }

    /// Receive generic reply
    ///
    /// This is required because a reply could be any of these at any moment
    pub fn receive(&mut self) -> Result<ClientCommand> {
        receive_command(&mut self.reader)
    }

    /// Request state update
    /// use `receive()` to get the reply
    pub fn request_state(&mut self) -> Result<()> {
        let command = RequestCommand::State;

        send_command(&mut self.writer, &command)
    }

    /// Persist changes to registry for current user
    pub fn persist(monitors: &[Monitor]) -> Result<()> {
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        let key = r"SOFTWARE\VirtualDisplayDriver";

        let mut reg_key = hklm.open_subkey_with_flags(key, KEY_WRITE);

        // if open failed, try to create key and subkey
        if let Err(e) = reg_key {
            error!("Failed opening {key}: {e:?}");
            reg_key = hklm.create_subkey(key).map(|(key, _)| key);

            if let Err(e) = reg_key {
                error!("Failed creating {key}: {e:?}");
                return Err(e)?;
            }
        }

        let reg_key = reg_key.unwrap();

        let data = serde_json::to_string(monitors)?;

        reg_key.set_value("data", &data)?;

        Ok(())
    }
}

fn send_command(writer: &mut NamedPipeClientWriter, command: &impl Serialize) -> Result<()> {
    // Create a vector with the full message, then send it as a single
    // write. This is required because the pipe is in message mode.
    let mut message = serde_json::to_vec(command)?;
    message.push(EOF);
    writer.write_all(&message)?;
    writer.flush()?;

    Ok(())
}

fn receive_command<T: DeserializeOwned>(reader: &mut NamedPipeClientReader) -> Result<T> {
    let mut msg_buf = Vec::with_capacity(4096);
    let mut buf = vec![0; 4096];

    loop {
        let Ok(size) = reader.read(&mut buf) else {
            break;
        };

        msg_buf.extend_from_slice(&buf[..size]);

        if msg_buf.last().is_some_and(|&byte| byte == EOF) {
            // pop off EOF
            msg_buf.pop();

            break;
        }
    }

    // in the following we assume we always get a fully formed message, e.g. no multiple messages

    // interior EOF is not valid
    assert!(
        !msg_buf.contains(&EOF),
        "interior eof detected, this is a bug; msg: {msg_buf:?}"
    );

    let command = serde_json::from_slice(&msg_buf)?;

    Ok(command)
}
