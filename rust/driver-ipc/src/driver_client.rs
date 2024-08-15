use std::collections::HashSet;

use tokio::{sync::watch, task};
use tokio_stream::{Stream, StreamExt};

use crate::*;

/// Abstraction layer over [Client].
///
/// It manages its own state. Changing this state does not affect the driver
/// directly. You must call [DriverClient::notify] to send changes to the
/// driver. To make your changes persistent across reboots, call
/// [DriverClient::persist]. To synchronize this object with the driver, you
/// must call [DriverClient::refresh_state]. The state will not be updated
/// automatically.
#[derive(Debug)]
pub struct DriverClient {
    client: Client,
    state_rx: watch::Receiver<Vec<Monitor>>,
    state: Vec<Monitor>,
}

impl DriverClient {
    /// Connect to driver on pipe with default name.
    ///
    /// The default name is [DEFAULT_PIPE_NAME]
    pub async fn new() -> Result<Self, error::InitError> {
        Self::new_with(DEFAULT_PIPE_NAME).await
    }

    /// Connect to driver on pipe with specified name.
    ///
    /// `name` is ONLY the {name} portion of \\.\pipe\{name}.
    pub async fn new_with(name: &str) -> Result<Self, error::InitError> {
        let client = Client::connect_to(name).await?;

        let current_state = client.request_state().await?;

        let (state_tx, state_rx) = watch::channel(current_state.clone());

        let mut stream = client.receive_events();

        task::spawn(async move {
            while let Some(event) = stream.next().await {
                if let Ok(EventCommand::Changed(value)) = event {
                    if state_tx.send(value).is_err() {
                        // Client was dropped, stop listening
                        break;
                    }
                }
            }
        });

        Ok(Self {
            client,
            state_rx,
            state: current_state,
        })
    }

    /// Get the ID of a monitor using a query.
    ///
    /// ## Query syntax
    ///
    /// The query can either be a monitor name or an ID.
    ///
    /// The name has precedence over the ID.
    ///
    /// ### Example
    /// ```ignore
    /// // Monitors are:
    /// //   { id: 0, name: Some("foo") }
    /// //   { id: 1, name: Some("bar") }
    /// //   { id: 2, name: Some("1") }
    ///
    /// assert_eq!(client.find_id("foo"), Some(0));
    /// assert_eq!(client.find_id("bar"), Some(1));
    /// assert_eq!(client.find_id("1"), Some(2));
    /// assert_eq!(client.find_id("0"), Some(0));
    /// assert_eq!(client.find_id("baz"), None);
    /// ```
    pub fn find_id(&self, query: &str) -> Option<Id> {
        let id = query.parse::<Id>();

        for monitor in self.monitors().iter() {
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

    /// Manually synchronize with the driver.
    pub fn refresh_state(&mut self) -> &[Monitor] {
        self.state = self.state_rx.borrow().clone();

        &self.state
    }

    /// Returns a stream of continuous events from the driver.
    ///
    /// This stream will always reflect the real state of the driver, regardless
    /// of who changed its state. This means, if it is changed by another
    /// process, this stream will still be updated.
    ///
    /// Note: If multiple copies of this client exist (using
    /// [DriverClient::duplicate]), the returned stream will only be closed
    /// after all copies are dropped.
    pub fn receive_events(&self) -> impl Stream<Item = Result<EventCommand, error::ReceiveError>> {
        self.client.receive_events()
    }

    /// Get the current monitor state stored inside this client.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn monitors(&self) -> &[Monitor] {
        &self.state
    }

    /// Replace all monitors.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    pub fn set_monitors(&mut self, monitors: &[Monitor]) -> Result<(), error::DuplicateError> {
        mons_have_duplicates(monitors)?;

        self.state = monitors.to_owned();
        Ok(())
    }

    /// Replace an existing monitor. The monitor is identified by its ID.
    ///
    /// Returns an error if the monitor does not exist.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn replace_monitor(&mut self, monitor: Monitor) -> Result<(), error::MonNotFound> {
        match self.state.iter_mut().find(|m| m.id == monitor.id) {
            Some(m) => {
                *m = monitor;
                Ok(())
            }
            None => Err(error::MonNotFound(monitor.id)),
        }
    }

    /// Send the current client state to the driver.
    ///
    /// State changes of the client are not automatically sent to the driver.
    /// You must manually call this method to send changes to the driver.
    pub async fn notify(&mut self) -> Result<(), error::SendError> {
        self.client.notify(&self.state).await
    }

    /// Find the monitor with the given ID.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn find_monitor(&self, id: Id) -> Option<&Monitor> {
        self.state.iter().find(|monitor| monitor.id == id)
    }

    /// Find the monitor matched by the given query.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn find_monitor_query(&self, query: &str) -> Option<&Monitor> {
        let id = self.find_id(query)?;
        self.find_monitor(id)
    }

    /// TODO: may leave client in an invalid state
    ///
    /// Find a monitor by ID and call `cb` with a mutable reference to it.
    ///
    /// Note: Any changes do not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn find_monitor_mut<R>(&mut self, id: Id, cb: impl FnOnce(&mut Monitor) -> R) -> Option<R> {
        let monitor = self.state.iter_mut().find(|monitor| monitor.id == id)?;

        let r = cb(monitor);

        mons_have_duplicates(&self.state).ok()?;

        Some(r)
    }

    /// Find a monitor by ID and return a mutable reference to it.
    ///
    /// Does not check for duplicates (since this is not easy when returning a
    /// mut reference) Caller agrees they will make sure there are no duplicates.
    ///
    /// Despite the "unchecked" part of this name, this is a safe method.
    ///
    /// Note: Any changes do not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn find_monitor_mut_unchecked(&mut self, id: Id) -> Option<&mut Monitor> {
        self.state.iter_mut().find(|monitor| monitor.id == id)
    }

    /// Find the monitor matched by the given query and call `cb` with a mutable
    /// reference to it.
    ///
    /// Note: Any changes do not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn find_monitor_mut_query<R>(
        &mut self,
        query: &str,
        cb: impl FnOnce(&mut Monitor) -> R,
    ) -> Option<R> {
        let id = self.find_id(query)?;
        self.find_monitor_mut(id, cb)
    }

    /// Find the monitor matching the given query.
    ///
    /// Does not check for duplicates (since this is not easy when returning a
    /// mut reference) Caller agrees they will make sure there are no duplicates.
    ///
    /// Despite the "unchecked" part of this name, this is a safe method.
    ///
    /// Note: Any changes do not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn find_monitor_mut_query_unchecked(&mut self, query: &str) -> Option<&mut Monitor> {
        let id = self.find_id(query)?;
        self.find_monitor_mut_unchecked(id)
    }

    /// Write client state to the registry for current user.
    ///
    /// Next time the driver is started, it will load this state from the
    /// registry. This might be after a reboot or a driver restart.
    pub fn persist(&self) -> Result<(), error::PersistError> {
        Client::persist(&self.state)
    }

    /// Get the closest available free ID.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    ///
    /// Note: Duplicate monitors are ignored when send to the Driver using
    /// [DriverClient::notify].
    pub fn new_id(&self, preferred_id: Option<Id>) -> Option<Id> {
        let existing_ids = self
            .state
            .iter()
            .map(|monitor| monitor.id)
            .collect::<HashSet<_>>();

        if let Some(id) = preferred_id {
            if existing_ids.contains(&id) {
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

    /// Remove monitors by id.
    ///
    /// Silently skips IDs that do not exist.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn remove(&mut self, ids: &[Id]) {
        self.state.retain(|mon| !ids.contains(&mon.id));
    }

    /// Remove all monitors matched by the given queries.
    ///
    /// Returns an error if a query does not match any monitor.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn remove_query(
        &mut self,
        queries: &[impl AsRef<str>],
    ) -> Result<(), error::QueryNotFound> {
        let mut ids = Vec::new();
        for id in queries {
            if let Some(id) = self.find_id(id.as_ref()) {
                ids.push(id);
                continue;
            }

            return Err(error::QueryNotFound(id.as_ref().to_owned()));
        }

        self.remove(&ids);
        Ok(())
    }

    /// Remove all monitors.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    pub fn remove_all(&mut self) {
        self.state.clear();
    }

    /// Add a new monitor.
    ///
    /// Returns an error if a monitor with this ID already exists, or if the
    /// monitor is invalid. A monitor is invalid if it has duplicate modes, or
    /// if any of its modes has duplicate refresh rates.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn add(&mut self, monitor: Monitor) -> Result<(), error::DuplicateError> {
        if self.state.iter().any(|mon| mon.id == monitor.id) {
            return Err(error::DuplicateError::Monitor(monitor.id));
        }
        mon_has_duplicates(&monitor)?;

        self.state.push(monitor);

        Ok(())
    }

    /// Set enabled state of all monitors with the given IDs.
    ///
    /// Silently skips incorrect IDs.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn set_enabled(&mut self, ids: &[Id], enabled: bool) {
        for mon in self.state.iter_mut() {
            if ids.contains(&mon.id) {
                mon.enabled = enabled;
                continue;
            }
        }
    }

    /// Set enabled state of all monitors matched by the given queries.
    ///
    /// Returns an error if a query does not match any monitor.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn set_enabled_query(
        &mut self,
        queries: &[impl AsRef<str>],
        enabled: bool,
    ) -> Result<(), error::QueryNotFound> {
        let mut ids = Vec::new();
        for id in queries {
            if let Some(id) = self.find_id(id.as_ref()) {
                ids.push(id);
                continue;
            }

            return Err(error::QueryNotFound(id.as_ref().to_owned()));
        }

        self.set_enabled(&ids, enabled);
        Ok(())
    }

    /// Add a mode to the monitor with the given ID.
    ///
    /// Returns an error if the monitor does not exist, or if the mode already
    /// exists on that monitor, or if the mode is invalid. A mode is invalid if
    /// it has duplicate refresh rates.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn add_mode(&mut self, id: Id, mode: Mode) -> Result<(), error::AddModeError> {
        let Some(mon) = self.state.iter_mut().find(|mon| mon.id == id) else {
            return Err(error::AddModeError::MonNotFound(id));
        };

        match mode_has_duplicates(&mode, id) {
            Ok(_) => Ok(()),
            Err(error::DuplicateError::RefreshRate(rr, w, h, id)) => {
                Err(error::AddModeError::DupRefreshRate(rr, w, h, id))
            }
            Err(_) => unreachable!(),
        }?;

        if mon
            .modes
            .iter()
            .any(|_mode| _mode.height == mode.height && _mode.width == mode.width)
        {
            return Err(error::AddModeError::DupMode(id, mode.width, mode.height));
        }

        mon.modes.push(mode);

        Ok(())
    }

    /// Add a mode to the a monitor matched by the given query.
    ///
    /// Returns an error if the monitor cannot be found, or if the mode already
    /// exists on that monitor, or if the mode is invalid. A mode is invalid if
    /// it has duplicate refresh rates.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn add_mode_query(
        &mut self,
        query: &str,
        mode: Mode,
    ) -> Result<(), error::AddModeQueryError> {
        let id = self
            .find_id(query)
            .ok_or_else(|| error::AddModeQueryError::QueryNotFound(query.to_owned()))?;

        match self.add_mode(id, mode) {
            Ok(_) => Ok(()),
            Err(error::AddModeError::MonNotFound(_)) => {
                unreachable!("Mon must exist")
            }
            Err(error::AddModeError::DupMode(w, h, id)) => {
                Err(error::AddModeQueryError::DupMode(id, w, h))
            }
            Err(error::AddModeError::DupRefreshRate(rr, w, h, id)) => {
                Err(error::AddModeQueryError::DupRefreshRate(rr, w, h, id))
            }
        }?;

        Ok(())
    }

    /// Remove a mode from the monitor with the given ID.
    ///
    /// Returns an error if the monitor does not exist. If the mode does not
    /// exist, it is silently skipped.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn remove_mode(
        &mut self,
        id: Id,
        resolution: (u32, u32),
    ) -> Result<(), error::MonNotFound> {
        let Some(mon) = self.state.iter_mut().find(|mon| mon.id == id) else {
            return Err(error::MonNotFound(id));
        };

        mon.modes
            .retain(|mode| !(mode.width == resolution.0 && mode.height == resolution.1));

        Ok(())
    }

    /// Remove a mode from a monitor matched by the given query.
    ///
    /// Returns an error if the monitor cannot be found. If the mode does not
    /// exist, it is silently skipped.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn remove_mode_query(
        &mut self,
        query: &str,
        resolution: (u32, u32),
    ) -> Result<(), error::QueryNotFound> {
        let id = self
            .find_id(query)
            .ok_or_else(|| error::QueryNotFound(query.to_owned()))?;

        self.remove_mode(id, resolution)
            .expect("Monitor must exist");

        Ok(())
    }

    /// Returns a copy of this client with it's own independent state.
    ///
    /// Changes to one client will not affect the other.
    ///
    /// Note: Event receivers created with [DriverClient::receive_events] will
    /// only be closed after all copies are dropped, regardless of which client
    /// was used to create the receiver.
    pub fn duplicate(&self) -> Self {
        Self {
            client: self.client.clone(),
            state_rx: self.state_rx.clone(),
            state: self.state.clone(),
        }
    }
}

fn mons_have_duplicates(monitors: &[Monitor]) -> Result<(), error::DuplicateError> {
    let mut monitor_iter = monitors.iter();
    while let Some(monitor) = monitor_iter.next() {
        let duplicate_id = monitor_iter.clone().any(|b| monitor.id == b.id);
        if duplicate_id {
            return Err(error::DuplicateError::Monitor(monitor.id));
        }

        mon_has_duplicates(monitor)?;
    }

    Ok(())
}

fn mon_has_duplicates(monitor: &Monitor) -> Result<(), error::DuplicateError> {
    let mut mode_iter = monitor.modes.iter();
    while let Some(mode) = mode_iter.next() {
        let duplicate_mode = mode_iter
            .clone()
            .any(|m| mode.height == m.height && mode.width == m.width);
        if duplicate_mode {
            return Err(error::DuplicateError::Mode(
                mode.width,
                mode.height,
                monitor.id,
            ));
        }

        mode_has_duplicates(mode, monitor.id)?;
    }

    Ok(())
}

fn mode_has_duplicates(mode: &Mode, id: Id) -> Result<(), error::DuplicateError> {
    let mut refresh_iter = mode.refresh_rates.iter().copied();
    while let Some(rr) = refresh_iter.next() {
        let duplicate_rr = refresh_iter.clone().any(|r| rr == r);
        if duplicate_rr {
            return Err(error::DuplicateError::RefreshRate(
                rr,
                mode.width,
                mode.height,
                id,
            ));
        }
    }

    Ok(())
}

pub mod error {
    use super::*;
    pub use crate::client::error::*;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum DuplicateError {
        #[error("Duplicate monitor with ID {0}")]
        Monitor(Id),
        #[error("Duplicate mode {0}x{1} on monitor {2}")]
        Mode(u32, u32, Id),
        #[error("Duplicate refresh rate {0} on mode {1}x{2} on monitor {3}")]
        RefreshRate(u32, u32, u32, Id),
    }

    #[derive(Debug, Error)]
    #[error("Query not found: {0}")]
    pub struct QueryNotFound(pub String);

    #[derive(Debug, Error)]
    #[error("Monitor not found: {0}")]
    pub struct MonNotFound(pub Id);

    /// Error returned from [DriverClient::add_mode].
    #[derive(Debug, Error)]
    pub enum AddModeError {
        #[error("Monitor not found: {0}")]
        MonNotFound(Id),
        #[error("Duplicate mode {1}x{2} on monitor {0}")]
        DupMode(Id, u32, u32),
        #[error("Duplicate refresh rate {0} on mode {1}x{2} on monitor {3}")]
        DupRefreshRate(u32, u32, u32, Id),
    }

    /// Error returned from [DriverClient::add_mode_query].
    #[derive(Debug, Error)]
    pub enum AddModeQueryError {
        #[error("Query not found: {0}")]
        QueryNotFound(String),
        #[error("Duplicate mode {1}x{2} on monitor {0}")]
        DupMode(Id, u32, u32),
        #[error("Duplicate refresh rate {0} on mode {1}x{2} on monitor {3}")]
        DupRefreshRate(u32, u32, u32, Id),
    }

    /// Error returned from [DriverClient::new] and [DriverClient::new_with].
    #[derive(Debug, Error)]
    pub enum InitError {
        #[error("Failed to connect to driver: {0}")]
        Connect(#[from] ConnectionError),
        #[error("Failed to request state: {0}")]
        RequestState(#[from] RequestError),
    }
}
