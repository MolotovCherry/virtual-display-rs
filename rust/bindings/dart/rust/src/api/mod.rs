pub mod client;
pub mod mock;

pub use client::*;
pub use mock::*;

pub use driver_ipc::Mode;
pub use driver_ipc::Monitor;

use flutter_rust_bridge::frb;

#[frb(mirror(Monitor), dart_metadata=("freezed"))]
struct _Monitor {
    pub id: u32,
    pub name: Option<String>,
    pub enabled: bool,
    pub modes: Vec<driver_ipc::Mode>,
}

#[frb(mirror(Mode), dart_metadata=("freezed"))]
struct _Mode {
    pub width: u32,
    pub height: u32,
    pub refresh_rates: Vec<u32>,
}
