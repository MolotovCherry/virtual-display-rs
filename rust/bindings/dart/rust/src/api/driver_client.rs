#![allow(clippy::doc_markdown)]

use driver_ipc::Monitor;
use flutter_rust_bridge::frb;
use tokio_stream::StreamExt;

use crate::frb_generated::StreamSink;

use super::client;

mod ipc {
    pub use driver_ipc::driver_client::*;
    pub use driver_ipc::*;
}

/// Abstraction layer over [Client].
///
/// It manages its own state. Changing this state does not affect the driver
/// directly. You must call [DriverClient.notify] to send changes to the
/// driver. To make your changes persistent across reboots, call
/// [DriverClient.persist]. To synchronize this object with the driver, you
/// must call [DriverClient.refreshState]. The state will not be updated
/// automatically.
#[frb(opaque)]
pub struct DriverClient(ipc::DriverClient);

impl DriverClient {
    /// Connect to driver on pipe with default name.
    ///
    /// You can optionally specify the name of the named pipe to connect to. The
    /// default name is "virtualdisplaydriver"
    pub async fn connect(pipe_name: Option<String>) -> Result<Self, InitError> {
        match pipe_name {
            Some(pipe_name) => Ok(Self(ipc::DriverClient::new_with(&pipe_name).await?)),
            None => Ok(Self(ipc::DriverClient::new().await?)),
        }
    }

    /// Manually synchronize with the driver.
    pub fn refresh_state(&mut self) -> Vec<Monitor> {
        self.0.refresh_state().to_owned()
    }

    /// Returns a stream of continuous events from the driver.
    ///
    /// This stream will always reflect the real state of the driver, regardless
    /// of who changed its state. This means, if it is changed by another
    /// process, this stream will still be updated.
    #[allow(clippy::unused_async)] // Must be async, because we need a tokio reactor.
    pub async fn receive_events(
        &self,
        sink: StreamSink<Vec<ipc::Monitor>>,
    ) -> Result<(), client::ReceiveError> {
        let mut stream = self.0.receive_events();
        tokio::task::spawn(async move {
            while let Some(v) = stream.next().await {
                let result = match v {
                    Ok(ipc::EventCommand::Changed(monitors)) => sink.add(monitors),
                    Err(err) => sink.add_error(client::ReceiveError::from(err)),
                    Ok(_) => continue,
                };

                if result.is_err() {
                    break;
                }
            }
        });
        Ok(())
    }

    /// Get the current monitor state stored inside this client.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient.refreshState].
    #[frb(getter, sync)]
    pub fn state(&self) -> Vec<Monitor> {
        self.0.monitors().to_owned()
    }

    /// Send the current client state to the driver.
    ///
    /// State changes of the client are not automatically sent to the driver.
    /// You must manually call this method to send changes to the driver.
    pub async fn notify(&mut self) -> Result<(), client::SendError> {
        self.0.notify().await?;
        Ok(())
    }

    /// Find the monitor with the given ID.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient.refreshState].
    #[must_use]
    #[frb(sync)]
    pub fn find_monitor(&self, id: u32) -> Option<Monitor> {
        self.0.find_monitor(id).cloned()
    }

    /// Replace all monitors.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient.notify] to send these changes to the driver.
    #[frb(sync)]
    pub fn set_monitors(&mut self, monitors: &[Monitor]) -> Result<(), DuplicateError> {
        self.0.set_monitors(monitors)?;
        Ok(())
    }

    /// Replace an existing monitor. The monitor is identified by its ID.
    ///
    /// Throws [MonitorNotFoundError] if the monitor does not exist.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient.notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient.refreshState].
    #[frb(sync)]
    pub fn replace_monitor(&mut self, monitor: Monitor) -> Result<(), MonitorNotFoundError> {
        self.0.replace_monitor(monitor)?;
        Ok(())
    }

    /// Remove monitors by id.
    ///
    /// Silently skips IDs that do not exist.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient.notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient.refreshState].
    #[frb(sync)]
    pub fn remove(&mut self, ids: &[u32]) {
        self.0.remove(ids);
    }

    /// Remove all monitors.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient.notify] to send these changes to the driver.
    #[frb(sync)]
    pub fn remove_all(&mut self) {
        self.0.remove_all();
    }

    /// Add a new monitor.
    ///
    /// Returns an error if a monitor with this ID already exists, or if the
    /// monitor is invalid. A monitor is invalid if it has duplicate modes, or
    /// if any of its modes has duplicate refresh rates.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient.notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient.refreshState].
    #[frb(sync)]
    pub fn add(&mut self, monitor: Monitor) -> Result<(), DuplicateError> {
        self.0.add(monitor)?;
        Ok(())
    }

    /// Set enabled state of all monitors with the given IDs.
    ///
    /// Silently skips incorrect IDs.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient.notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient.refreshState].
    #[frb(sync)]
    pub fn set_enabled(&mut self, ids: &[u32], enabled: bool) {
        self.0.set_enabled(ids, enabled);
    }

    /// Add a mode to the monitor with the given ID.
    ///
    /// Returns an error if the monitor does not exist, or if the mode already
    /// exists on that monitor, or if the mode is invalid. A mode is invalid if
    /// it has duplicate refresh rates.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient.notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient.refreshState].
    #[frb(sync)]
    pub fn add_mode(&mut self, id: u32, mode: ipc::Mode) -> Result<(), AddModeError> {
        self.0.add_mode(id, mode)?;
        Ok(())
    }

    /// Remove a mode from the monitor with the given ID.
    ///
    /// Returns an error if the monitor does not exist. If the mode does not
    /// exist, it is silently skipped.
    ///
    /// Note: This does not affect the driver. Manually call
    /// [DriverClient.notify] to send these changes to the driver.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient.refreshState].
    #[frb(sync)]
    pub fn remove_mode(
        &mut self,
        id: u32,
        resolution: (u32, u32),
    ) -> Result<(), MonitorNotFoundError> {
        self.0.remove_mode(id, resolution)?;
        Ok(())
    }

    /// Get the closest available free ID.
    ///
    /// Note: Client state might be stale. To synchronize with the driver,
    /// manually call [DriverClient.refreshState].
    ///
    /// Note: Duplicate monitors are ignored when send to the Driver using
    /// [DriverClient.notify].
    #[must_use]
    #[frb(sync)]
    pub fn new_id(&self, preferred_id: Option<u32>) -> Option<u32> {
        self.0.new_id(preferred_id)
    }

    /// Write client state to the registry for current user.
    ///
    /// Next time the driver is started, it will load this state from the
    /// registry. This might be after a reboot or a driver restart.
    pub fn persist(&self) -> Result<(), client::PersistError> {
        self.0.persist()?;
        Ok(())
    }
}

#[frb(dart_code = "
    @override
    String toString() => 'Failed to initialize DriverClient: $inner';
")]
pub enum InitError {
    Connect { inner: client::ConnectionError },
    RequestState { inner: client::RequestError },
}

impl From<ipc::error::InitError> for InitError {
    fn from(value: ipc::error::InitError) -> Self {
        match value {
            ipc::error::InitError::Connect(inner) => InitError::Connect {
                inner: inner.into(),
            },
            ipc::error::InitError::RequestState(inner) => InitError::RequestState {
                inner: inner.into(),
            },
        }
    }
}

#[frb(dart_code = "
    @override
    String toString() => switch (this) {
        DuplicateError_Monitor(:final id) => 'Monitor with id $id already exists',
        DuplicateError_Mode(:final monitorId, :final width, :final height) => 'Mode ${width}x$height already exists on monitor $monitorId',
        DuplicateError_RefreshRate(:final monitorId, :final width, :final height, :final refreshRate) => 'Refresh rate $refreshRate already exists on mode ${width}x$height on monitor $monitorId',
    };
")]
pub enum DuplicateError {
    Monitor {
        id: u32,
    },
    Mode {
        monitor_id: u32,
        width: u32,
        height: u32,
    },
    RefreshRate {
        monitor_id: u32,
        width: u32,
        height: u32,
        refresh_rate: u32,
    },
}

impl From<ipc::error::DuplicateError> for DuplicateError {
    fn from(value: ipc::error::DuplicateError) -> Self {
        match value {
            ipc::error::DuplicateError::Monitor(id) => DuplicateError::Monitor { id },
            ipc::error::DuplicateError::Mode(width, height, monitor_id) => DuplicateError::Mode {
                monitor_id,
                width,
                height,
            },
            ipc::error::DuplicateError::RefreshRate(refresh_rate, width, height, monitor_id) => {
                DuplicateError::RefreshRate {
                    monitor_id,
                    width,
                    height,
                    refresh_rate,
                }
            }
        }
    }
}

#[frb(dart_code = "
    @override
    String toString() => 'Monitor with id $id not found';
")]
pub struct MonitorNotFoundError {
    pub id: u32,
}

impl From<ipc::error::MonNotFound> for MonitorNotFoundError {
    fn from(value: ipc::error::MonNotFound) -> Self {
        MonitorNotFoundError { id: value.0 }
    }
}

#[frb(dart_code = "
    @override
    String toString() => switch (this) {
        AddModeError_MonitorNotFound(:final id) => 'Monitor with id $id not found',
        AddModeError_ModeExists(:final monitorId, :final width, :final height) => 'Mode ${width}x$height already exists on monitor $monitorId',
        AddModeError_RefreshRateExists(:final monitorId, :final width, :final height, :final refreshRate) => 'Refresh rate $refreshRate already exists on mode ${width}x$height on monitor $monitorId',
    };
")]
pub enum AddModeError {
    MonitorNotFound {
        id: u32,
    },
    ModeExists {
        monitor_id: u32,
        width: u32,
        height: u32,
    },
    RefreshRateExists {
        monitor_id: u32,
        width: u32,
        height: u32,
        refresh_rate: u32,
    },
}

impl From<ipc::error::AddModeError> for AddModeError {
    fn from(value: ipc::error::AddModeError) -> Self {
        match value {
            ipc::error::AddModeError::MonNotFound(id) => AddModeError::MonitorNotFound { id },
            ipc::error::AddModeError::DupMode(monitor_id, width, height) => {
                AddModeError::ModeExists {
                    monitor_id,
                    width,
                    height,
                }
            }
            ipc::error::AddModeError::DupRefreshRate(refresh_rate, width, height, monitor_id) => {
                AddModeError::RefreshRateExists {
                    monitor_id,
                    width,
                    height,
                    refresh_rate,
                }
            }
        }
    }
}
