pub use driver_ipc::{Mode, Monitor};
use flutter_rust_bridge::frb;

use crate::frb_generated::StreamSink;

#[frb(mirror(Monitor), dart_metadata=("freezed"))]
pub struct _Monitor {
    pub id: u32,
    pub name: Option<String>,
    pub enabled: bool,
    pub modes: Vec<Mode>,
}

#[frb(mirror(Mode), dart_metadata=("freezed"))]
pub struct _Mode {
    pub width: u32,
    pub height: u32,
    pub refresh_rates: Vec<u32>,
}

#[frb(opaque)]
pub struct VirtualDisplayDriver {}

impl VirtualDisplayDriver {
    #[frb(sync)]
    pub fn new(pipe_name: Option<String>) -> VirtualDisplayDriver // Result<VirtualDisplayDriver, ...>
    {
        VirtualDisplayDriver {}
    }

    /// Get the current state of the driver.
    #[frb(getter)]
    pub fn state(&self) -> Vec<Monitor> // -> Result<Vec<Monitor>, ...>
    {
        // Can return std::result::Result or anyhow::Result. The latter will be
        // converted into a generic AnyhowException on the Dart side. The Error
        // trait is ignored. eyre::Result does not work.
        todo!("state")
    }

    /// Stream of state changes.
    ///
    /// Updates whenever the state of the driver changes. It does not matter
    /// from which process the change is requested. It will always reflect the
    /// current state of the driver.
    ///
    /// After calling, will instantly emit the current state of the driver.
    #[frb(getter)]
    pub fn stream(&self, sink: StreamSink<Vec<Monitor>>) // -> Result<(), ...>
    {
        todo!("stream")
    }

    /// Set the state of the provided monitors.
    ///
    /// Each monitor with a matching ID will be updated to the provided state.
    pub fn set_monitors(&self, monitors: Vec<Monitor>) // -> Result<(), ...>
    {
        // assert only existing monitors
        todo!("set_monitors")
    }

    /// Set the state of the monitor with the provided ID.
    ///
    /// Only the provided properties will be updated.
    pub fn set_monitor(
        &self,
        id: u32,
        enabled: Option<bool>,
        name: Option<String>,
        modes: Option<Vec<Mode>>,
    ) // -> Result<(), ...>
    {
        // assert only existing monitor
        todo!("set_monitor_enabled")
    }

    /// Add a new monitor to the driver.
    pub fn add_monitor(&self, name: Option<String>, enabled: bool, modes: Vec<Mode>)
    // -> Result<(), ...>
    {
        todo!("add_monitor")
    }

    /// Remove monitors from the driver.
    pub fn remove_monitors(&self, ids: Vec<u32>) // -> Result<(), ...>
    {
        todo!("remove_monitors")
    }

    /// Remove all monitors from the driver.
    pub fn remove_all_monitors(&self) // -> Result<(), ...>
    {
        todo!("remove_all_monitors")
    }
}
