use std::{collections::HashSet, mem::ManuallyDrop, thread};

use eyre::bail;
use log::error;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_WRITE},
    RegKey,
};

use crate::{Client, ClientCommand, Id, Mode, Monitor, ReplyCommand};

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

        let state = Self::get_state(&mut client)?;

        Ok(Self { client, state })
    }

    fn get_state(client: &mut ManuallyDrop<Client>) -> eyre::Result<Vec<Monitor>> {
        client.request_state()?;

        while let Ok(command) = client.receive() {
            match command {
                ClientCommand::Reply(ReplyCommand::State(state)) => {
                    return Ok(state);
                }

                _ => continue,
            }
        }

        bail!("Failed to get request state");
    }

    /// Get the ID of a monitor using a string. The string can be either the name
    /// of the monitor, or the ID.
    ///
    /// This ID can then be used in other api calls.
    ///
    /// This search first searches name, then id of each monitor in consecutive order
    /// So it's possible a name could be "5", and a monitor Id 5 next wouldn't be found
    /// since a name matched before
    ///
    /// Ex) "my-mon-name", "5"
    /// In the above example, this will search for a monitor of name "my-mon-name", "5", or
    /// an ID of 5
    pub fn find_id(&self, query: &str) -> eyre::Result<Id> {
        let id = query.parse::<Id>();

        for monitor in &self.state {
            if let Some(name) = monitor.name.as_deref() {
                if name == query {
                    return Ok(monitor.id);
                }
            }

            if let Ok(id) = id {
                if monitor.id == id {
                    return Ok(monitor.id);
                }
            }
        }

        bail!("Monitor matching query \"{query}\" not found");
    }

    /// Manually refresh internal state with latest driver changes
    pub fn refresh_state(&mut self) -> eyre::Result<&[Monitor]> {
        self.state = Self::get_state(&mut self.client)?;

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

    /// Replace a monitor
    /// Determines which monitor to replace based on the ID
    pub fn replace_monitor(&mut self, monitor: Monitor) -> eyre::Result<()> {
        for state_monitor in self.state.iter_mut() {
            if state_monitor.id == monitor.id {
                *state_monitor = monitor;
                return Ok(());
            }
        }

        bail!("Monitor ID {} not found", monitor.id);
    }

    /// All changes done are in-memory only. They are only applied when you run `notify()``,
    /// and only saved when you run `persist()``
    ///
    /// Send current state to driver
    pub fn notify(&mut self) -> eyre::Result<()> {
        self.client.notify(&self.state)
    }

    /// Find a monitor by ID.
    pub fn find_monitor(&self, query: Id) -> eyre::Result<&Monitor> {
        let monitor_by_id = self.state.iter().find(|monitor| monitor.id == query);
        if let Some(monitor) = monitor_by_id {
            return Ok(monitor);
        }

        bail!("virtual monitor with ID {query} not found");
    }

    /// Find a monitor by query.
    pub fn find_monitor_query(&self, query: &str) -> eyre::Result<&Monitor> {
        let id = self.find_id(query)?;
        self.find_monitor(id)
    }

    /// Find a monitor by ID and return mutable reference to it.
    pub fn find_monitor_mut(&mut self, query: Id) -> eyre::Result<&mut Monitor> {
        let monitor_by_id = self.state.iter_mut().find(|monitor| monitor.id == query);
        if let Some(monitor) = monitor_by_id {
            return Ok(monitor);
        }

        bail!("virtual monitor with ID {query:?} not found");
    }

    /// Find a monitor by query.
    pub fn find_monitor_mut_query(&mut self, query: &str) -> eyre::Result<&mut Monitor> {
        let id = self.find_id(query)?;
        self.find_monitor_mut(id)
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
    pub fn remove(&mut self, ids: &[Id]) {
        self.state.retain(|mon| !ids.contains(&mon.id));
    }

    /// Remove monitors by query
    pub fn remove_query(&mut self, queries: &[impl AsRef<str>]) -> eyre::Result<()> {
        let mut ids = Vec::new();
        for id in queries {
            if let Ok(id) = self.find_id(id.as_ref()) {
                ids.push(id);
                continue;
            }

            bail!("Monitor query \"{}\" not found", id.as_ref());
        }

        self.remove(&ids);
        Ok(())
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
    ///
    /// @return: tells you if all monitors in list were found
    pub fn set_enabled(&mut self, ids: &[Id], enabled: bool) {
        for mon in self.state.iter_mut() {
            if ids.contains(&mon.id) {
                mon.enabled = enabled;
                continue;
            }
        }
    }

    /// Enable monitors by query
    pub fn set_enabled_query(
        &mut self,
        queries: &[impl AsRef<str>],
        enabled: bool,
    ) -> eyre::Result<()> {
        let mut ids = Vec::new();
        for id in queries {
            if let Ok(id) = self.find_id(id.as_ref()) {
                ids.push(id);
                continue;
            }

            bail!("Monitor query \"{}\" not found", id.as_ref());
        }

        self.set_enabled(&ids, enabled);
        Ok(())
    }

    /// Add a mode to monitor
    pub fn add_mode(&mut self, id: Id, mode: Mode) -> eyre::Result<()> {
        let Some(mon) = self.state.iter_mut().find(|mon| mon.id == id) else {
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

    /// Add a mode to monitor by query
    pub fn add_mode_query(&mut self, query: &str, mode: Mode) -> eyre::Result<()> {
        let id = self.find_id(query)?;
        self.add_mode(id, mode)
    }

    /// Remove a monitor mode
    pub fn remove_mode(&mut self, id: Id, resolution: (u32, u32)) -> eyre::Result<()> {
        let Some(mon) = self.state.iter_mut().find(|mon| mon.id == id) else {
            bail!("Monitor id {id:?} not found");
        };

        mon.modes
            .retain(|mode| !(mode.width == resolution.0 && mode.height == resolution.1));

        Ok(())
    }

    /// Add a mode to monitor by query
    pub fn remove_mode_query(&mut self, query: &str, resolution: (u32, u32)) -> eyre::Result<()> {
        let id = self.find_id(query)?;
        self.remove_mode(id, resolution)
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
