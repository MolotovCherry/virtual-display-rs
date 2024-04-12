use std::{collections::HashSet, sync::OnceLock};

use tokio::{
    sync::mpsc::{channel, Sender},
    task,
};

use crate::{Client, ClientError, EventCommand, Id, IpcError, Mode, Monitor, ReplyCommand, Result};

// used to terminate existing receivers
static RECEIVER_SHUTDOWN_TOKEN: std::sync::Mutex<OnceLock<Sender<()>>> =
    std::sync::Mutex::new(OnceLock::new());

/// Extra API over Client which allows nice fancy things
#[derive(Debug)]
pub struct DriverClient {
    client: Client,
    state: Vec<Monitor>,
}

impl DriverClient {
    /// connect to default driver name
    pub async fn new() -> Result<Self> {
        let mut client = Client::connect().await?;

        let state = Self::_get_state(&mut client).await?;

        Ok(Self { client, state })
    }

    /// specify pipe name to connect to
    pub async fn new_with(name: &str) -> Result<Self> {
        let mut client = Client::connect_to(name).await?;

        let state = Self::_get_state(&mut client).await?;

        Ok(Self { client, state })
    }

    async fn _get_state(client: &mut Client) -> Result<Vec<Monitor>> {
        client.request_state().await?;

        loop {
            match client.receive_reply(true).await {
                Some(ReplyCommand::State(state)) => {
                    return Ok(state);
                }

                _ => {
                    continue;
                }
            }
        }
    }

    async fn get_state(&mut self) -> Result<Vec<Monitor>> {
        Self::_get_state(&mut self.client).await
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
    pub fn find_id(&self, query: &str) -> Option<Id> {
        let id = query.parse::<Id>();

        for monitor in &self.state {
            if let Some(name) = monitor.name.as_deref() {
                if name == query {
                    return Some(monitor.id);
                }
            }

            if let Ok(id) = id {
                if monitor.id == id {
                    return Some(monitor.id);
                }
            }
        }

        None
    }

    /// Manually refresh internal state with latest driver changes
    pub async fn refresh_state(&mut self) -> Result<&[Monitor]> {
        self.state = self.get_state().await?;

        Ok(&self.state)
    }

    /// Supply a callback used to receive commands from the driver
    ///
    /// This only allows one receiver at a time. Setting a new cb will also terminate existing reciever
    ///
    /// Note: DriverClient DOES NOT do any hidden state changes! Only calling proper api will change internal state.
    ///       driver state IS NOT updated on its own when Event commands are received!
    ///       if you want to update internal state, call set_monitors on DriverClient in your callback
    ///       to properly handle it!
    pub async fn set_event_receiver(&mut self, cb: impl Fn(EventCommand) + Send + 'static) {
        // stop currently running task if any exist
        let token = RECEIVER_SHUTDOWN_TOKEN.lock().unwrap().take();
        if let Some(sender) = token {
            sender.send(()).await.unwrap();
        }

        let (tx, mut rx) = channel(1);
        RECEIVER_SHUTDOWN_TOKEN.lock().unwrap().set(tx).unwrap();

        let client = self.client.clone();
        task::spawn(async move {
            loop {
                tokio::select! {
                    cmd = client.receive_event() => {
                        cb(cmd);
                    }

                    // shutdown signal
                    _ = rx.recv() => {
                        break;
                    }
                }
            }
        });
    }

    /// Terminate a receiver without setting a new one
    pub async fn terminate_event_receiver(&self) {
        // stop currently running task if any exist
        let token = RECEIVER_SHUTDOWN_TOKEN.lock().unwrap().take();
        if let Some(sender) = token {
            sender.send(()).await.unwrap();
        }
    }

    /// Get the current monitor state
    pub fn monitors(&self) -> &[Monitor] {
        &self.state
    }

    /// Set the internal monitor state
    pub fn set_monitors(&mut self, monitors: &[Monitor]) -> Result<()> {
        mons_have_duplicates(monitors)?;

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
    pub async fn notify(&mut self) -> Result<()> {
        self.client.notify(&self.state).await
    }

    /// Find a monitor by ID.
    pub fn find_monitor(&self, id: Id) -> Option<&Monitor> {
        let monitor_by_id = self.state.iter().find(|monitor| monitor.id == id);
        monitor_by_id
    }

    /// Find a monitor by query.
    pub fn find_monitor_query(&self, query: &str) -> Option<&Monitor> {
        let id = self.find_id(query)?;
        self.find_monitor(id)
    }

    /// Find a monitor by ID and return mutable reference to it.
    pub fn find_monitor_mut<R>(&mut self, id: Id, cb: impl FnOnce(&mut Monitor) -> R) -> Option<R> {
        let monitor = self.state.iter_mut().find(|monitor| monitor.id == id)?;

        let r = cb(monitor);

        mons_have_duplicates(&self.state).ok()?;

        Some(r)
    }

    /// Find a monitor by ID and return mutable reference to it.
    ///
    /// Does not do checking to validate there are no duplicates (since this is not easy when returning a mut reference)
    /// Caller agrees they will make sure there are no duplicates
    ///
    /// Despite the "unchecked" part of this name, this is a safe method
    pub fn find_monitor_mut_unchecked(&mut self, id: Id) -> Option<&mut Monitor> {
        let monitor_by_id = self.state.iter_mut().find(|monitor| monitor.id == id);
        monitor_by_id
    }

    /// Find a monitor by query.
    pub fn find_monitor_mut_query<R>(
        &mut self,
        query: &str,
        cb: impl FnOnce(&mut Monitor) -> R,
    ) -> Option<R> {
        let id = self.find_id(query)?;
        self.find_monitor_mut(id, cb)
    }

    /// Find a monitor by query.
    ///
    /// Does not do checking to validate there are no duplicates (since this is not easy when returning a mut reference)
    /// Caller agrees they will make sure there are no duplicates
    ///
    /// Despite the "unchecked" part of this name, this is a safe method
    pub fn find_monitor_mut_query_unchecked(&mut self, query: &str) -> Option<&mut Monitor> {
        let id = self.find_id(query)?;
        self.find_monitor_mut_unchecked(id)
    }

    /// Persist changes to registry for current user
    pub fn persist(&self) -> Result<()> {
        Client::persist(&self.state)
    }

    /// Get the closest available free ID. Note that if internal state is stale, this may result in a duplicate ID
    /// which the driver will ignore when you notify it of changes
    pub fn new_id(&self, preferred_id: Option<Id>) -> Option<Id> {
        let existing_ids = self
            .state
            .iter()
            .map(|monitor| monitor.id)
            .collect::<HashSet<_>>();

        if let Some(id) = preferred_id {
            if !existing_ids.contains(&id) {
                return None;
            }

            Some(id)
        } else {
            #[allow(clippy::maybe_infinite_iter)]
            let new_id = (0..)
                .find(|id| !existing_ids.contains(id))
                .expect("failed to get a new ID");
            Some(new_id)
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
            if let Some(id) = self.find_id(id.as_ref()) {
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
        mon_has_duplicates(&monitor)?;

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
            if let Some(id) = self.find_id(id.as_ref()) {
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

        mode_has_duplicates(&mode, id)?;

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
        let id = self
            .find_id(query)
            .ok_or_else(|| IpcError::Client(ClientError::QueryNotFound(query.to_owned())))?;

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

    /// Remove monitor mode by query
    pub fn remove_mode_query(&mut self, query: &str, resolution: (u32, u32)) -> Result<()> {
        let id = self
            .find_id(query)
            .ok_or_else(|| IpcError::Client(ClientError::QueryNotFound(query.to_owned())))?;

        self.remove_mode(id, resolution)
    }
}

fn mons_have_duplicates(monitors: &[Monitor]) -> Result<()> {
    let mut monitor_iter = monitors.iter();
    while let Some(monitor) = monitor_iter.next() {
        let duplicate_id = monitor_iter.clone().any(|b| monitor.id == b.id);
        if duplicate_id {
            return Err(ClientError::DupMon(monitor.id).into());
        }

        mon_has_duplicates(monitor)?;
    }

    Ok(())
}

fn mon_has_duplicates(monitor: &Monitor) -> Result<()> {
    let mut mode_iter = monitor.modes.iter();
    while let Some(mode) = mode_iter.next() {
        let duplicate_mode = mode_iter
            .clone()
            .any(|m| mode.height == m.height && mode.width == m.width);
        if duplicate_mode {
            return Err(ClientError::DupMode(mode.width, mode.height, monitor.id).into());
        }

        mode_has_duplicates(mode, monitor.id)?;
    }

    Ok(())
}

fn mode_has_duplicates(mode: &Mode, id: Id) -> Result<()> {
    let mut refresh_iter = mode.refresh_rates.iter().copied();
    while let Some(rr) = refresh_iter.next() {
        let duplicate_rr = refresh_iter.clone().any(|r| rr == r);
        if duplicate_rr {
            return Err(ClientError::DupRefreshRate(rr, mode.width, mode.height, id).into());
        }
    }

    Ok(())
}
