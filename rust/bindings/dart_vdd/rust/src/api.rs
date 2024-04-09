use driver_ipc::{ClientCommand, Dimen, DriverClient, EventCommand, Id, RefreshRate, Result};
pub use driver_ipc::{Mode, Monitor};
use flutter_rust_bridge::frb;

use crate::frb_generated::StreamSink;

#[frb(mirror(Monitor), dart_metadata=("freezed"))]
pub struct _Monitor {
    pub id: Id,
    pub name: Option<String>,
    pub enabled: bool,
    pub modes: Vec<Mode>,
}

#[frb(mirror(Mode), dart_metadata=("freezed"))]
pub struct _Mode {
    pub width: Dimen,
    pub height: Dimen,
    pub refresh_rates: Vec<RefreshRate>,
}

#[frb(opaque)]
pub struct VirtualDisplayDriver {
    client: DriverClient,
}

impl VirtualDisplayDriver {
    #[frb(sync)]
    pub fn new(_pipe_name: Option<String>) -> Result<VirtualDisplayDriver> {
        let vdd = VirtualDisplayDriver {
            client: DriverClient::new()?,
        };

        Ok(vdd)
    }

    /// Get the current state of the driver.
    #[frb(getter)]
    pub fn state(&self) -> Vec<Monitor> {
        self.client.monitors().to_owned()
    }

    /// Stream of state changes.
    ///
    /// Updates whenever the state of the driver changes. It does not matter
    /// from which process the change is requested. It will always reflect the
    /// current state of the driver.
    ///
    /// After calling, will instantly emit the current state of the driver.
    #[frb(getter)]
    pub fn stream<F: FnOnce() + Send + 'static>(&self, sink: StreamSink<Vec<Monitor>>) {
        self.client.set_receiver(None::<F>, move |command| {
            if let ClientCommand::Event(EventCommand::Changed(data)) = command {
                if let Err(_e) = sink.add(data) {
                    // do something with err? hmm
                }
            }
        });
    }

    /// Set the state of the provided monitors.
    ///
    /// Each monitor with a matching ID will be updated to the provided state.
    pub fn set_monitors(&mut self, monitors: &[Monitor]) -> Result<()> {
        self.client.set_monitors(&monitors)
    }

    /// Set the state of the monitor with the provided ID.
    ///
    /// Only the provided properties will be updated.
    pub fn set_monitor(
        &mut self,
        id: u32,
        enabled: Option<bool>,
        name: Option<String>,
        modes: Option<Vec<Mode>>,
    ) -> Result<()> {
        self.client.find_monitor_mut(id, |monitor| {
            if let Some(enabled) = enabled {
                monitor.enabled = enabled;
            }

            monitor.name = name;

            if let Some(modes) = modes {
                monitor.modes = modes;
            }
        })?;

        Ok(())
    }

    /// Add a new monitor to the driver.
    pub fn add_monitor(
        &mut self,
        name: Option<String>,
        enabled: bool,
        modes: Vec<Mode>,
    ) -> Result<()> {
        let monitor = Monitor {
            id: self.client.new_id(None)?,
            name,
            enabled,
            modes,
        };

        self.client.add(monitor);

        Ok(())
    }

    /// Remove monitors from the driver.
    pub fn remove_monitors(&mut self, ids: &[u32]) {
        self.client.remove(&ids);
    }

    /// Remove all monitors from the driver.
    pub fn remove_all_monitors(&mut self) {
        self.client.remove_all();
    }

    /// Push in-memory changes to driver.
    pub fn notify(&mut self) -> Result<()> {
        self.client.notify()
    }

    /// Persist in-memory changes to user settings
    pub fn persist(&mut self) -> Result<()> {
        self.client.persist()
    }
}
