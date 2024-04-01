use std::{collections::HashSet, mem::ManuallyDrop, thread};

use eyre::bail;
use log::error;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_WRITE},
    RegKey,
};

use crate::{Client, ClientCommand, Id, Mode, Monitor, ReplyCommand};

#[derive(Debug, Clone)]
pub enum Identifier {
    Id(Id),
    Name(String),
}

/// Extra API over Client which allows nice fancy things
#[derive(Debug)]
pub struct DriverClient {
    // I'll handle dropping of this field manually
    client: ManuallyDrop<Client>,
    state: Vec<Monitor>,
}

impl DriverClient {
    pub fn new() -> eyre::Result<Self> {
        let mut client = ManuallyDrop::new(Client::connect()?);

        client.request_state()?;

        let state = 'outer: loop {
            while let Ok(command) = client.receive() {
                match command {
                    ClientCommand::Reply(ReplyCommand::State(state)) => {
                        break 'outer state;
                    }

                    _ => continue,
                }
            }
        };

        Ok(Self { client, state })
    }

    /// Manually refresh internal state with latest driver changes
    pub fn refresh_state(&mut self) -> eyre::Result<&[Monitor]> {
        self.client.request_state()?;

        let state = 'outer: loop {
            while let Ok(command) = self.client.receive() {
                match command {
                    ClientCommand::Reply(ReplyCommand::State(state)) => {
                        break 'outer state;
                    }

                    _ => continue,
                }
            }
        };

        self.state = state;

        Ok(&self.state)
    }

    /// Supply a callback used to receive commands from the driver
    ///
    /// Note: DriverClient DOES NOT do any hidden state changes! Only calling proper api will change internal state.
    ///       driver state IS NOT updated on its own when Event commands are received!
    ///       if you want to update internal state, call set_monitors on DriverClient in your callback
    ///       to properly handle it!
    pub fn set_receiver(&self, cb: impl Fn(ClientCommand) + Send + 'static) {
        let mut client = self.client.clone();
        thread::spawn(move || {
            while let Ok(command) = client.receive() {
                cb(command);
            }
        });
    }

    /// Get the current monitor state
    pub fn monitors(&self) -> &[Monitor] {
        &self.state
    }

    /// Set the internal monitor state
    pub fn set_monitors(&mut self, monitors: &[Monitor]) -> eyre::Result<()> {
        has_duplicates(monitors)?;

        self.state = monitors.to_owned();
        Ok(())
    }

    /// Send current state to driver
    pub fn notify(&mut self) -> eyre::Result<()> {
        self.client.notify(&self.state)
    }

    /// Find a monitor by ID or name.
    pub fn find_monitor(&self, query: Identifier) -> eyre::Result<Monitor> {
        let copy = query.clone();

        if let Identifier::Id(id) = query {
            let monitor_by_id = self.state.iter().find(|monitor| monitor.id == id);
            if let Some(monitor) = monitor_by_id {
                return Ok(monitor.clone());
            }
        }

        if let Identifier::Name(query) = query {
            let monitor_by_name = self
                .state
                .iter()
                .find(|monitor| monitor.name.as_deref().is_some_and(|name| name == query));
            if let Some(monitor) = monitor_by_name {
                return Ok(monitor.clone());
            }
        }

        bail!("virtual monitor with ID {copy:?} not found");
    }

    /// Find a monitor by ID or name.
    pub fn find_monitor_mut(&mut self, query: Identifier) -> eyre::Result<&mut Monitor> {
        let copy = query.clone();

        match query {
            Identifier::Id(id) => {
                let monitor_by_id = self.state.iter_mut().find(|monitor| monitor.id == id);
                if let Some(monitor) = monitor_by_id {
                    return Ok(monitor);
                }
            }

            Identifier::Name(query) => {
                let monitor_by_name = self
                    .state
                    .iter_mut()
                    .find(|monitor| monitor.name.as_deref().is_some_and(|name| name == query));
                if let Some(monitor) = monitor_by_name {
                    return Ok(monitor);
                }
            }
        }

        bail!("virtual monitor with ID {copy:?} not found");
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

    /// Get the closest available free ID. Note that if internal state is stale, this may result in a duplicate ID
    /// which the driver will ignore when you notify it of changes
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

    /// Remove monitors by id
    pub fn remove(&mut self, ids: &[Identifier]) {
        for id in ids {
            match id {
                Identifier::Id(id) => self.state.retain(|mon| mon.id != *id),

                Identifier::Name(name) => self
                    .state
                    .retain(|mon| mon.name.as_deref().is_some_and(|_name| _name != name)),
            }
        }
    }

    /// Remove all monitors
    pub fn remove_all(&mut self) {
        self.state.clear();
    }

    /// Add new monitor
    pub fn add(&mut self, monitor: Monitor) -> eyre::Result<()> {
        if self.state.iter().any(|mon| mon.id == monitor.id) {
            bail!("Monitor {} already exists", monitor.id);
        }

        self.state.push(monitor);

        Ok(())
    }

    /// Enable monitors by ID
    ///
    /// Silently skips incorrect IDs
    pub fn enable(&mut self, ids: &[Identifier]) {
        for id in ids {
            if let Some(mon) = self.state.iter_mut().find(|mon| match id {
                Identifier::Id(id) => mon.id == *id,
                Identifier::Name(name) => mon.name.as_deref().is_some_and(|_name| _name == name),
            }) {
                mon.enabled = true;
            }
        }
    }

    /// Disable monitors by ID
    ///
    /// Silently skips incorrect IDs
    pub fn disable(&mut self, ids: &[Identifier]) {
        for id in ids {
            if let Some(mon) = self.state.iter_mut().find(|mon| match id {
                Identifier::Id(id) => mon.id == *id,
                Identifier::Name(name) => mon.name.as_deref().is_some_and(|_name| _name == name),
            }) {
                mon.enabled = false;
            }
        }
    }

    /// Add a mode to monitor
    pub fn add_mode(&mut self, id: Identifier, mode: Mode) -> eyre::Result<()> {
        let Some(mon) = self.state.iter_mut().find(|mon| match &id {
            Identifier::Id(id) => mon.id == *id,
            Identifier::Name(name) => mon.name.as_deref().is_some_and(|_name| _name == name),
        }) else {
            bail!("Monitor id {id:?} not found");
        };

        if mon
            .modes
            .iter()
            .any(|_mode| _mode.height == mode.height && _mode.width == mode.width)
        {
            bail!("Mode {}x{} already exists", mode.width, mode.height);
        }

        let iter = mode.refresh_rates.iter().copied();

        for (i, &rr) in mode.refresh_rates.iter().enumerate() {
            if iter.clone().skip(i).any(|_rr| _rr == rr) {
                bail!("Detected duplicate refresh rate {rr}");
            }
        }

        mon.modes.push(mode);

        Ok(())
    }

    /// Remove a monitor mode
    pub fn remove_mode(&mut self, id: Identifier, resolution: (u32, u32)) -> eyre::Result<()> {
        let Some(mon) = self.state.iter_mut().find(|mon| match &id {
            Identifier::Id(id) => mon.id == *id,
            Identifier::Name(name) => mon.name.as_deref().is_some_and(|_name| _name == name),
        }) else {
            bail!("Monitor id {id:?} not found");
        };

        mon.modes
            .retain(|mode| !(mode.width == resolution.0 && mode.height == resolution.1));

        Ok(())
    }
}

impl Drop for DriverClient {
    fn drop(&mut self) {
        use std::os::windows::io::AsRawHandle as _;
        // get raw pipe handle. reader/writer doesn't matter, they're all the same handle
        let handle = self.client.writer.as_raw_handle();

        // manually close handle so that ReadFile stops blocking our thread
        unsafe {
            _ = CloseHandle(HANDLE(handle as _));
        }
    }
}

fn has_duplicates(monitors: &[Monitor]) -> eyre::Result<()> {
    let mut monitor_iter = monitors.iter();
    while let Some(monitor) = monitor_iter.next() {
        let duplicate_id = monitor_iter.clone().any(|b| monitor.id == b.id);
        if duplicate_id {
            bail!("Found duplicate monitor id {}", monitor.id);
        }

        let mut mode_iter = monitor.modes.iter();
        while let Some(mode) = mode_iter.next() {
            let duplicate_mode = mode_iter
                .clone()
                .any(|m| mode.height == m.height && mode.width == m.width);
            if duplicate_mode {
                bail!(
                    "Found duplicate mode {}x{} on monitor {}",
                    mode.width,
                    mode.height,
                    monitor.id
                );
            }

            let mut refresh_iter = mode.refresh_rates.iter().copied();
            while let Some(rr) = refresh_iter.next() {
                let duplicate_rr = refresh_iter.clone().any(|r| rr == r);
                if duplicate_rr {
                    bail!(
                        "Found duplicate refresh rate {rr} on mode {}x{} for monitor {}",
                        mode.width,
                        mode.height,
                        monitor.id
                    );
                }
            }
        }
    }

    Ok(())
}
