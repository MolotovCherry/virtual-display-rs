use super::RUNTIME;
use crate::{DriverClient as AsyncDriverClient, EventCommand, Id, Mode, Monitor, Result};

/// Extra API over Client which allows nice fancy things
#[derive(Debug)]
pub struct DriverClient(AsyncDriverClient);

impl DriverClient {
    /// connect to default driver name
    pub fn new() -> Result<Self> {
        let client = RUNTIME.block_on(AsyncDriverClient::new());
        client.map(Self)
    }

    /// specify pipe name to connect to
    pub fn new_with(name: &str) -> Result<Self> {
        let client = RUNTIME.block_on(AsyncDriverClient::new_with(name));
        client.map(Self)
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
        self.0.find_id(query)
    }

    /// Manually refresh internal state with latest driver changes
    pub fn refresh_state(&mut self) -> Result<&[Monitor]> {
        RUNTIME.block_on(self.0.refresh_state())
    }

    /// Supply a callback used to receive commands from the driver
    ///
    /// This only allows one receiver at a time. Setting a new cb will also terminate existing reciever
    ///
    /// Note: DriverClient DOES NOT do any hidden state changes! Only calling proper api will change internal state.
    ///       driver state IS NOT updated on its own when Event commands are received!
    ///       if you want to update internal state, call set_monitors on DriverClient in your callback
    ///       to properly handle it!
    pub fn set_event_receiver(&mut self, cb: impl Fn(EventCommand) + Send + 'static) {
        RUNTIME.block_on(self.0.set_event_receiver(cb))
    }

    /// Terminate a receiver without setting a new one
    pub fn terminate_event_receiver(&self) {}

    /// Get the current monitor state
    pub fn monitors(&self) -> &[Monitor] {
        self.0.monitors()
    }

    /// Set the internal monitor state
    pub fn set_monitors(&mut self, monitors: &[Monitor]) -> Result<()> {
        self.0.set_monitors(monitors)
    }

    /// Replace a monitor
    /// Determines which monitor to replace based on the ID
    pub fn replace_monitor(&mut self, monitor: Monitor) -> Result<()> {
        self.0.replace_monitor(monitor)
    }

    /// All changes done are in-memory only. They are only applied when you run `notify()``,
    /// and only saved when you run `persist()``
    ///
    /// Send current state to driver
    pub fn notify(&mut self) -> Result<()> {
        RUNTIME.block_on(self.0.notify())
    }

    /// Find a monitor by ID.
    pub fn find_monitor(&self, id: Id) -> Option<&Monitor> {
        self.0.find_monitor(id)
    }

    /// Find a monitor by query.
    pub fn find_monitor_query(&self, query: &str) -> Option<&Monitor> {
        self.0.find_monitor_query(query)
    }

    /// Find a monitor by ID and return mutable reference to it.
    pub fn find_monitor_mut<R>(&mut self, id: Id, cb: impl FnOnce(&mut Monitor) -> R) -> Option<R> {
        self.0.find_monitor_mut(id, cb)
    }

    /// Find a monitor by ID and return mutable reference to it.
    ///
    /// Does not do checking to validate there are no duplicates (since this is not easy when returning a mut reference)
    /// Caller agrees they will make sure there are no duplicates
    ///
    /// Despite the "unchecked" part of this name, this is a safe method
    pub fn find_monitor_mut_unchecked(&mut self, id: Id) -> Option<&mut Monitor> {
        self.0.find_monitor_mut_unchecked(id)
    }

    /// Find a monitor by query.
    pub fn find_monitor_mut_query<R>(
        &mut self,
        query: &str,
        cb: impl FnOnce(&mut Monitor) -> R,
    ) -> Option<R> {
        self.0.find_monitor_mut_query(query, cb)
    }

    /// Find a monitor by query.
    ///
    /// Does not do checking to validate there are no duplicates (since this is not easy when returning a mut reference)
    /// Caller agrees they will make sure there are no duplicates
    ///
    /// Despite the "unchecked" part of this name, this is a safe method
    pub fn find_monitor_mut_query_unchecked(&mut self, query: &str) -> Option<&mut Monitor> {
        self.0.find_monitor_mut_query_unchecked(query)
    }

    /// Persist changes to registry for current user
    pub fn persist(&self) -> Result<()> {
        self.0.persist()
    }

    /// Get the closest available free ID. Note that if internal state is stale, this may result in a duplicate ID
    /// which the driver will ignore when you notify it of changes
    pub fn new_id(&self, preferred_id: Option<Id>) -> Option<Id> {
        self.0.new_id(preferred_id)
    }

    /// Remove monitors by id
    pub fn remove(&mut self, ids: &[Id]) {
        self.0.remove(ids)
    }

    /// Remove monitors by query
    pub fn remove_query(&mut self, queries: &[impl AsRef<str>]) -> Result<()> {
        self.0.remove_query(queries)
    }

    /// Remove all monitors
    pub fn remove_all(&mut self) {
        self.0.remove_all()
    }

    /// Add new monitor
    pub fn add(&mut self, monitor: Monitor) -> Result<()> {
        self.0.add(monitor)
    }

    /// Enable monitors by ID
    ///
    /// Silently skips incorrect IDs
    ///
    /// @return: tells you if all monitors in list were found
    pub fn set_enabled(&mut self, ids: &[Id], enabled: bool) {
        self.0.set_enabled(ids, enabled)
    }

    /// Enable monitors by query
    pub fn set_enabled_query(&mut self, queries: &[impl AsRef<str>], enabled: bool) -> Result<()> {
        self.0.set_enabled_query(queries, enabled)
    }

    /// Add a mode to monitor
    pub fn add_mode(&mut self, id: Id, mode: Mode) -> Result<()> {
        self.0.add_mode(id, mode)
    }

    /// Add a mode to monitor by query
    pub fn add_mode_query(&mut self, query: &str, mode: Mode) -> Result<()> {
        self.0.add_mode_query(query, mode)
    }

    /// Remove a monitor mode
    pub fn remove_mode(&mut self, id: Id, resolution: (u32, u32)) -> Result<()> {
        self.0.remove_mode(id, resolution)
    }

    /// Remove monitor mode by query
    pub fn remove_mode_query(&mut self, query: &str, resolution: (u32, u32)) -> Result<()> {
        self.0.remove_mode_query(query, resolution)
    }
}
