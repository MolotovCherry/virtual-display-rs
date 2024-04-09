use std::{collections::HashSet, mem::ManuallyDrop, thread};

use windows::Win32::Foundation::{CloseHandle, HANDLE};

use crate::{
    Client, ClientCommand, ClientError, Id, IpcError, Mode, Monitor, ReplyCommand, Result,
};

/// Extra API over Client which allows nice fancy things
#[derive(Debug)]
pub struct DriverClient {
    // I'll handle dropping of this field manually
    client: ManuallyDrop<Client>,
    state: Vec<Monitor>,
}

impl DriverClient {
    pub fn new() -> Result<Self> {
        let mut client = ManuallyDrop::new(Client::connect()?);

        let state = Self::get_state(&mut client)?;

        Ok(Self { client, state })
    }

    fn get_state(client: &mut ManuallyDrop<Client>) -> Result<Vec<Monitor>> {
        client.request_state()?;

        while let Ok(command) = client.receive() {
            match command {
                ClientCommand::Reply(ReplyCommand::State(state)) => {
                    return Ok(state);
                }

                _ => continue,
            }
        }

        Err(IpcError::RequestState)
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
    pub fn find_id(&self, query: &str) -> Result<Id> {
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

        Err(ClientError::QueryNotFound(query.to_owned()).into())
    }

    /// Manually refresh internal state with latest driver changes
    pub fn refresh_state(&mut self) -> Result<&[Monitor]> {
        self.state = Self::get_state(&mut self.client)?;

        Ok(&self.state)
    }

    /// Supply a callback used to receive commands from the driver
    ///
    /// Note: DriverClient DOES NOT do any hidden state changes! Only calling proper api will change internal state.
    ///       driver state IS NOT updated on its own when Event commands are received!
    ///       if you want to update internal state, call set_monitors on DriverClient in your callback
    ///       to properly handle it!
    pub fn set_receiver(
        &self,
        init: Option<impl FnOnce() + Send + 'static>,
        cb: impl Fn(ClientCommand) + Send + 'static,
    ) {
        let mut client = self.client.clone();
        thread::spawn(move || {
            if let Some(init) = init {
                init();
            }

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
    pub fn set_monitors(&mut self, monitors: &[Monitor]) -> Result<()> {
        has_duplicates(monitors)?;

        self.state = monitors.to_owned();
        Ok(())
    }

    /// Replace a monitor
    /// Determines which monitor to replace based on the ID
    pub fn replace_monitor(&mut self, monitor: Monitor) -> Result<()> {
        for state_monitor in self.state.iter_mut() {
            if state_monitor.id == monitor.id {
                *state_monitor = monitor;
                return Ok(());
            }
        }

        Err(ClientError::MonNotFound(monitor.id).into())
    }

    /// All changes done are in-memory only. They are only applied when you run `notify()``,
    /// and only saved when you run `persist()``
    ///
    /// Send current state to driver
    pub fn notify(&mut self) -> Result<()> {
        self.client.notify(&self.state)
    }

    /// Find a monitor by ID.
    pub fn find_monitor(&self, id: Id) -> Result<&Monitor> {
        let monitor_by_id = self.state.iter().find(|monitor| monitor.id == id);
        if let Some(monitor) = monitor_by_id {
            return Ok(monitor);
        }

        Err(ClientError::MonNotFound(id).into())
    }

    /// Find a monitor by query.
    pub fn find_monitor_query(&self, query: &str) -> Result<&Monitor> {
        let id = self.find_id(query)?;
        self.find_monitor(id)
    }

    /// Find a monitor by ID and return mutable reference to it.
    pub fn find_monitor_mut(&mut self, id: Id) -> Result<&mut Monitor> {
        let monitor_by_id = self.state.iter_mut().find(|monitor| monitor.id == id);
        if let Some(monitor) = monitor_by_id {
            return Ok(monitor);
        }

        Err(ClientError::MonNotFound(id).into())
    }

    /// Find a monitor by query.
    pub fn find_monitor_mut_query(&mut self, query: &str) -> Result<&mut Monitor> {
        let id = self.find_id(query)?;
        self.find_monitor_mut(id)
    }

    /// Persist changes to registry for current user
    pub fn persist(&self) -> Result<()> {
        Client::persist(&self.state)
    }

    /// Get the closest available free ID. Note that if internal state is stale, this may result in a duplicate ID
    /// which the driver will ignore when you notify it of changes
    pub fn new_id(&self, preferred_id: Option<Id>) -> Result<Id> {
        let existing_ids = self
            .state
            .iter()
            .map(|monitor| monitor.id)
            .collect::<HashSet<_>>();

        if let Some(id) = preferred_id {
            if !existing_ids.contains(&id) {
                return Err(ClientError::MonExists(id).into());
            }

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
    pub fn remove_query(&mut self, queries: &[impl AsRef<str>]) -> Result<()> {
        let mut ids = Vec::new();
        for id in queries {
            if let Ok(id) = self.find_id(id.as_ref()) {
                ids.push(id);
                continue;
            }

            return Err(ClientError::QueryNotFound(id.as_ref().to_owned()).into());
        }

        self.remove(&ids);
        Ok(())
    }

    /// Remove all monitors
    pub fn remove_all(&mut self) {
        self.state.clear();
    }

    /// Add new monitor
    pub fn add(&mut self, monitor: Monitor) -> Result<()> {
        if self.state.iter().any(|mon| mon.id == monitor.id) {
            return Err(ClientError::MonExists(monitor.id).into());
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
    pub fn set_enabled_query(&mut self, queries: &[impl AsRef<str>], enabled: bool) -> Result<()> {
        let mut ids = Vec::new();
        for id in queries {
            if let Ok(id) = self.find_id(id.as_ref()) {
                ids.push(id);
                continue;
            }

            return Err(ClientError::QueryNotFound(id.as_ref().to_owned()).into());
        }

        self.set_enabled(&ids, enabled);
        Ok(())
    }

    /// Add a mode to monitor
    pub fn add_mode(&mut self, id: Id, mode: Mode) -> Result<()> {
        let Some(mon) = self.state.iter_mut().find(|mon| mon.id == id) else {
            return Err(ClientError::MonNotFound(id).into());
        };

        if mon
            .modes
            .iter()
            .any(|_mode| _mode.height == mode.height && _mode.width == mode.width)
        {
            return Err(ClientError::ModeExists(mode.width, mode.height).into());
        }

        let iter = mode.refresh_rates.iter().copied();

        for (i, &rr) in mode.refresh_rates.iter().enumerate() {
            if iter.clone().skip(i).any(|_rr| _rr == rr) {
                return Err(ClientError::RefreshRateExists(rr).into());
            }
        }

        mon.modes.push(mode);

        Ok(())
    }

    /// Add a mode to monitor by query
    pub fn add_mode_query(&mut self, query: &str, mode: Mode) -> Result<()> {
        let id = self.find_id(query)?;
        self.add_mode(id, mode)
    }

    /// Remove a monitor mode
    pub fn remove_mode(&mut self, id: Id, resolution: (u32, u32)) -> Result<()> {
        let Some(mon) = self.state.iter_mut().find(|mon| mon.id == id) else {
            return Err(ClientError::MonNotFound(id).into());
        };

        mon.modes
            .retain(|mode| !(mode.width == resolution.0 && mode.height == resolution.1));

        Ok(())
    }

    /// Add a mode to monitor by query
    pub fn remove_mode_query(&mut self, query: &str, resolution: (u32, u32)) -> Result<()> {
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

fn has_duplicates(monitors: &[Monitor]) -> Result<()> {
    let mut monitor_iter = monitors.iter();
    while let Some(monitor) = monitor_iter.next() {
        let duplicate_id = monitor_iter.clone().any(|b| monitor.id == b.id);
        if duplicate_id {
            return Err(ClientError::DupMon(monitor.id).into());
        }

        let mut mode_iter = monitor.modes.iter();
        while let Some(mode) = mode_iter.next() {
            let duplicate_mode = mode_iter
                .clone()
                .any(|m| mode.height == m.height && mode.width == m.width);
            if duplicate_mode {
                return Err(ClientError::DupMode(mode.width, mode.height, monitor.id).into());
            }

            let mut refresh_iter = mode.refresh_rates.iter().copied();
            while let Some(rr) = refresh_iter.next() {
                let duplicate_rr = refresh_iter.clone().any(|r| rr == r);
                if duplicate_rr {
                    return Err(ClientError::DupRefreshRate(
                        rr,
                        mode.width,
                        mode.height,
                        monitor.id,
                    )
                    .into());
                }
            }
        }
    }

    Ok(())
}
