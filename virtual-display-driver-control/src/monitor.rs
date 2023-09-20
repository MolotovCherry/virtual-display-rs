use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MonitorState {
    pub name: String,
    // whether monitor is enabled or not
    pub enabled: bool,
    pub monitor: Monitor,
    // Runtime state variables
    #[serde(skip)]
    pub monitor_window: bool,
    // input boxes for adding new monitor
    #[serde(skip)]
    pub tmp_w: u32,
    #[serde(skip)]
    pub tmp_h: u32,
    #[serde(skip)]
    pub tmp_r: u32,
    #[serde(skip)]
    pub tmp_add: u32,
}

pub trait RemovePending {
    fn remove_pending(self) -> Self;
}

impl RemovePending for Vec<MonitorState> {
    fn remove_pending(mut self) -> Self {
        for monitor in self.iter_mut() {
            monitor.monitor.remove_pending();
        }

        self
    }
}

// This is an internal monitor type made for ease of use in the app
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Monitor {
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<BTreeMap<String, MonitorMode>>,
}

impl Monitor {
    /// Remove all pending elements from
    pub fn remove_pending(&mut self) {
        if let Some(modes) = &mut self.modes {
            modes.retain(|_, mode| {
                mode.refresh_rates.retain(|rate| !rate.pending);

                !mode.pending
            })
        }
    }

    /// Mark all pending as fine
    pub fn clear_pending(&mut self) {
        if let Some(modes) = &mut self.modes {
            for mode in modes.values_mut() {
                if mode.pending {
                    mode.pending = false;
                }

                for rate in &mut mode.refresh_rates {
                    if rate.pending {
                        rate.pending = false;
                    }
                }
            }
        }
    }
}

// Nice and easy type to use in egui
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rates: Vec<RefreshRate>,
    #[serde(skip)]
    pub pending: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshRate {
    pub rate: u32,
    #[serde(skip)]
    pub pending: bool,
}

// And a lovely nice and easy converter to driver_ipc::Monitor!
impl From<Monitor> for driver_ipc::Monitor {
    fn from(value: Monitor) -> Self {
        driver_ipc::Monitor {
            id: value.id,
            modes: {
                let mut ipc_modes = Vec::new();

                if let Some(modes) = value.modes {
                    for (_, mode) in modes {
                        // do not send over pending decisions
                        if mode.pending {
                            continue;
                        }

                        for RefreshRate {
                            rate: refresh_rate,
                            pending,
                        } in mode.refresh_rates
                        {
                            // do not send over pending decisions
                            if pending {
                                continue;
                            }

                            ipc_modes.push(driver_ipc::MonitorMode {
                                width: mode.width,
                                height: mode.height,
                                refresh_rate,
                            });
                        }
                    }
                }

                ipc_modes
            },
        }
    }
}

pub trait IntoIpc {
    fn into_monitors(self) -> Vec<driver_ipc::Monitor>;
}

impl IntoIpc for Vec<MonitorState> {
    fn into_monitors(self) -> Vec<driver_ipc::Monitor> {
        self.into_iter().map(|i| i.monitor.into()).collect()
    }
}
