mod client;
mod core;
mod driver_client;
pub mod sync;
mod utils;

pub use client::Client;
pub use core::*;
pub use driver_client::DriverClient;

pub type Result<T> = std::result::Result<T, IpcError>;

#[derive(Debug, thiserror::Error)]
pub enum IpcError {
    #[error("failed to (de)serialize: {0}")]
    SerDe(#[from] serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("windows api error: {0}")]
    Win(#[from] windows::core::Error),
    #[error("{0}")]
    Client(#[from] ClientError),
    #[error("failed to get request state")]
    RequestState,
    #[error("failed to receive command")]
    Receive,
    #[error("failed to open pipe. is driver installed and working?\nerror: {0}")]
    ConnectionFailed(std::io::Error),
    #[error("channel closed")]
    SendFailed,
}

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("monitor id {0} not found")]
    MonNotFound(Id),
    #[error("monitor query \"{0}\" not found")]
    QueryNotFound(String),
    #[error("monitor {0} already exists")]
    MonExists(Id),
    #[error("mode {0}x{1} already exists")]
    ModeExists(Dimen, Dimen),
    #[error("detected duplicate refresh rate {0}")]
    RefreshRateExists(RefreshRate),
    #[error("found duplicate monitor id {0}")]
    DupMon(Id),
    #[error("found duplicate mode {0}x{1} on monitor {2}")]
    DupMode(Dimen, Dimen, Id),
    #[error("found duplicate refresh rate {0} on {1}x{2} for monitor {3}")]
    DupRefreshRate(RefreshRate, Dimen, Dimen, Id),
}
