use std::{collections::HashSet, io::Write as _};

use eyre::{bail, Context as _};
use log::error;
use serde::{de::DeserializeOwned, Serialize};
use win_pipes::{NamedPipeClientReader, NamedPipeClientWriter};
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_WRITE},
    RegKey,
};

use crate::{DriverCommand, Id, Monitor, ReplyCommand, RequestCommand};

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
                .create()
                .context("Failed to connect to Virtual Display Driver; please ensure the driver is installed and working")?;

        send_command(&mut writer, &RequestCommand::State)?;
        let state = receive_command::<ReplyCommand>(&mut reader)?;
        let ReplyCommand::State(state) = state;

        Ok(Self { writer, state })
    }

    pub fn monitors(&self) -> &[Monitor] {
        &self.state
    }

    /// Find a monitor by ID or name.
    pub fn find_monitor(&self, query: &str) -> eyre::Result<Monitor> {
        let query_id: Option<Id> = query.parse().ok();
        if let Some(query_id) = query_id {
            let monitor_by_id = self.state.iter().find(|monitor| monitor.id == query_id);
            if let Some(monitor) = monitor_by_id {
                return Ok(monitor.clone());
            }
        }

        let monitor_by_name = self
            .state
            .iter()
            .find(|monitor| monitor.name.as_deref().is_some_and(|name| name == query));
        if let Some(monitor) = monitor_by_name {
            return Ok(monitor.clone());
        }

        eyre::bail!("virtual monitor with ID {} not found", query);
    }

    /// Notifies driver of changes (additions/updates/removals)
    pub fn notify(&mut self, monitors: Vec<Monitor>) -> eyre::Result<()> {
        let command = DriverCommand::Notify(monitors);

        send_command(&mut self.writer, &command)?;

        Ok(())
    }

    /// Remove specific monitors by id
    pub fn remove(&mut self, ids: Vec<Id>) -> eyre::Result<()> {
        let command = DriverCommand::Remove(ids.clone());

        send_command(&mut self.writer, &command)?;
        self.state.retain(|mon| !ids.contains(&mon.id));

        Ok(())
    }

    /// Remove all monitors
    pub fn remove_all(&mut self) -> eyre::Result<()> {
        let command = DriverCommand::RemoveAll;

        send_command(&mut self.writer, &command)?;
        self.state.clear();

        Ok(())
    }

    /// Persist changes to registry for current user
    pub fn persist(&mut self) -> eyre::Result<()> {
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

        let Ok(data) = serde_json::to_string(&self.state) else {
            bail!("Failed to convert state to json");
        };

        if reg_key.set_value("data", &data).is_err() {
            bail!("Failed to save reg key");
        }

        Ok(())
    }

    pub fn new_id(&mut self, preferred_id: Option<Id>) -> eyre::Result<Id> {
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
