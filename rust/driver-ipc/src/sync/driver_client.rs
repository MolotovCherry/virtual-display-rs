use super::{client::EventsSubscription, RUNTIME};
use crate::{DriverClient as AsyncDriverClient, EventCommand, Id, Mode, Monitor, Result};

/// Abstraction layer over [Client].
///
/// It manages its own state. Changing this state does not affect the driver
/// directly. You must call [DriverClient::notify] to send changes to the
/// driver. To make your changes persistent across reboots, call
/// [DriverClient::persist]. To synchronize this object with the driver, you
/// must call [DriverClient::refresh_state]. The state will not be updated
/// automatically.
#[derive(Debug)]
pub struct DriverClient(AsyncDriverClient);

impl DriverClient {
    /// Connect to driver on pipe with default name.
    ///
    /// The default name is [DEFAULT_PIPE_NAME]
    pub fn new() -> Result<Self> {
        let client = RUNTIME.block_on(AsyncDriverClient::new());
        client.map(Self)
    }

    /// Connect to driver on pipe with specified name.
    ///
    /// `name` is ONLY the {name} portion of \\.\pipe\{name}.
    pub fn new_with(name: &str) -> Result<Self> {
        let client = RUNTIME.block_on(AsyncDriverClient::new_with(name));
        client.map(Self)
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
        self.0.find_id(query)
    }

    /// Manually synchronize with the driver.
    pub fn refresh_state(&mut self) -> Result<&[Monitor]> {
        self.0.refresh_state()
    }

    /// Add an event receiver to receive continuous events from the driver.
    ///
    /// This receiver will always reflect the real state of the driver,
    /// regardless of who changed its state. This means, if it is changed by
    /// another process, this receiver will still be updated.
    ///
    /// Returns an object that can be used to cancel the subscription.
    ///
    /// Note: The callback should return as soon as possible. It is called from
    /// the library's tokio runtime and blocks all other operations. In
    /// consequence, all other library events will be delayed until the callback
    /// returns.
    ///
    /// Note: If multiple copies of this client exist (using
    /// [DriverClient::duplicate]), the returned stream will only be closed
    /// after all copies are dropped.
    pub fn add_event_receiver(
        &self,
        cb: impl FnMut(EventCommand) + Send + std::panic::UnwindSafe + 'static,
    ) -> EventsSubscription {
        let stream = self.0.receive_events();
        EventsSubscription::start_subscriber(cb, stream)
    }

    /// Get the current monitor state stored inside this client.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn monitors(&self) -> &[Monitor] {
        self.0.monitors()
    }

    /// Replace all monitors.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    pub fn set_monitors(&mut self, monitors: &[Monitor]) -> Result<()> {
        self.0.set_monitors(monitors)
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
    pub fn replace_monitor(&mut self, monitor: Monitor) -> Result<()> {
        self.0.replace_monitor(monitor)
    }

    /// Send the current client state to the driver.
    ///
    /// State changes of the client are not automatically sent to the driver.
    /// You must manually call this method to send changes to the driver.
    pub fn notify(&mut self) -> Result<()> {
        RUNTIME.block_on(self.0.notify())
    }

    /// Find the monitor with the given ID.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn find_monitor(&self, id: Id) -> Option<&Monitor> {
        self.0.find_monitor(id)
    }

    /// Find the monitor matched by the given query.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn find_monitor_query(&self, query: &str) -> Option<&Monitor> {
        self.0.find_monitor_query(query)
    }

    /// Find a monitor by ID and call `cb` with a mutable reference to it.
    ///
    /// Note: Any changes do not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    pub fn find_monitor_mut<R>(&mut self, id: Id, cb: impl FnOnce(&mut Monitor) -> R) -> Option<R> {
        self.0.find_monitor_mut(id, cb)
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
        self.0.find_monitor_mut_unchecked(id)
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
        self.0.find_monitor_mut_query(query, cb)
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
        self.0.find_monitor_mut_query_unchecked(query)
    }

    /// Write client state to the registry for current user.
    ///
    /// Next time the driver is started, it will load this state from the
    /// registry. This might be after a reboot or a driver restart.
    pub fn persist(&self) -> Result<()> {
        self.0.persist()
    }

    /// Get the closest available free ID.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient::refresh_state].
    ///
    /// Note: Duplicate monitors are ignored when send to the Driver using
    /// [DriverClient::notify].
    pub fn new_id(&self, preferred_id: Option<Id>) -> Option<Id> {
        self.0.new_id(preferred_id)
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
        self.0.remove(ids)
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
    pub fn remove_query(&mut self, queries: &[impl AsRef<str>]) -> Result<()> {
        self.0.remove_query(queries)
    }

    /// Remove all monitors.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient::notify] to send these changes to the driver.
    pub fn remove_all(&mut self) {
        self.0.remove_all()
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
    pub fn add(&mut self, monitor: Monitor) -> Result<()> {
        self.0.add(monitor)
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
        self.0.set_enabled(ids, enabled)
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
    pub fn set_enabled_query(&mut self, queries: &[impl AsRef<str>], enabled: bool) -> Result<()> {
        self.0.set_enabled_query(queries, enabled)
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
    pub fn add_mode(&mut self, id: Id, mode: Mode) -> Result<()> {
        self.0.add_mode(id, mode)
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
    pub fn add_mode_query(&mut self, query: &str, mode: Mode) -> Result<()> {
        self.0.add_mode_query(query, mode)
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
    pub fn remove_mode(&mut self, id: Id, resolution: (u32, u32)) -> Result<()> {
        self.0.remove_mode(id, resolution)
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
    pub fn remove_mode_query(&mut self, query: &str, resolution: (u32, u32)) -> Result<()> {
        self.0.remove_mode_query(query, resolution)
    }

    /// Returns a copy of this client with it's own independent state.
    ///
    /// Changes to one client will not affect the other.
    ///
    /// Note: Event receivers created with [DriverClient::add_event_receiver] will
    /// only be closed after all copies are dropped, regardless of which client
    /// was used to create the receiver.
    pub fn duplicate(&self) -> Self {
        Self(self.0.duplicate())
    }
}
