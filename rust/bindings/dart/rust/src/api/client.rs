use flutter_rust_bridge::frb;
use tokio_stream::StreamExt;

use crate::frb_generated::StreamSink;

mod ipc {
    pub use driver_ipc::client::*;
    pub use driver_ipc::*;
}

/// Client for interacting with the Virtual Display Driver.
///
/// Connects via a named pipe to the driver.
///
/// You can send changes to the driver and receive continuous events from it.
#[frb(opaque)]
pub struct Client(ipc::Client);

impl Client {
    /// Connect to the driver.
    ///
    /// You can optionally specify the name of the named pipe to connect to. The
    /// default value is "virtualdisplaydriver".
    pub async fn connect(pipe_name: Option<String>) -> Result<Self, ConnectionError> {
        match pipe_name {
            Some(pipe_name) => Ok(Self(ipc::Client::connect_to(&pipe_name).await?)),
            None => Ok(Self(ipc::Client::connect().await?)),
        }
    }

    /// Send new state to the driver.
    pub async fn notify(&self, monitors: &[ipc::Monitor]) -> Result<(), SendError> {
        self.0.notify(monitors).await?;
        Ok(())
    }

    /// Remove all monitors with the specified IDs.
    pub async fn remove(&self, ids: &[u32]) -> Result<(), SendError> {
        self.0.remove(ids).await?;
        Ok(())
    }

    /// Remove all monitors.
    pub async fn remove_all(&self) -> Result<(), SendError> {
        self.0.remove_all().await?;
        Ok(())
    }

    /// Request the current state of the driver.
    ///
    /// Throws [RequestError.timeout] if the driver does not respond within 5
    /// seconds.
    pub async fn request_state(&self) -> Result<Vec<ipc::Monitor>, RequestError> {
        Ok(self.0.request_state().await?)
    }

    /// Receive continuous events from the driver.
    ///
    /// Only new events after calling this method are received.
    ///
    /// May be called multiple times.
    pub async fn receive_events(
        &self,
        sink: StreamSink<Vec<ipc::Monitor>>,
    ) -> Result<(), ReceiveError> {
        let mut stream = self.0.receive_events();
        while let Some(v) = stream.next().await {
            let result = match v {
                Ok(ipc::EventCommand::Changed(monitors)) => sink.add(monitors),
                Err(err) => sink.add_error(ReceiveError::from(err)),
                Ok(_) => continue,
            };

            if result.is_err() {
                return Ok(());
            }
        }
        Ok(())
    }

    /// Write `monitors` to the registry for current user.
    ///
    /// Next time the driver is started, it will load this state from the
    /// registry. This might be after a reboot or a driver restart.
    pub fn persist(monitors: &[ipc::Monitor]) -> Result<(), PersistError> {
        ipc::Client::persist(monitors)?;
        Ok(())
    }
}

#[frb(dart_code = "
    @override
    String toString() => switch (this) {
        ConnectionError_Failed(:final message) => 'Failed to open pipe: $message',
    };
")]
pub enum ConnectionError {
    Failed { message: String },
}

impl From<ipc::error::ConnectionError> for ConnectionError {
    fn from(err: ipc::error::ConnectionError) -> Self {
        match err {
            ipc::error::ConnectionError::Failed(err) => ConnectionError::Failed {
                message: err.to_string(),
            },
        }
    }
}

#[frb(dart_code = "
    @override
    String toString() => switch (this) {
        SendError_PipeBroken(:final message) => 'Failed to send message (pipe broken): $message',
    };
")]
pub enum SendError {
    PipeBroken { message: String },
}

impl From<ipc::error::SendError> for SendError {
    fn from(err: ipc::error::SendError) -> Self {
        match err {
            ipc::error::SendError::PipeBroken(err) => SendError::PipeBroken {
                message: err.to_string(),
            },
        }
    }
}

#[frb(dart_code = "
    @override
    String toString() => switch (this) {
        RequestError_Send(:final message) => 'Failed to send message (pipe broken): $message',
        RequestError_Receive(:final message) => 'Failed to receive message (pipe broken): $message',
        RequestError_Timeout(:final duration) => 'Did not get a response in time ($duration)',
    };
")]
pub enum RequestError {
    Send { message: String },
    Receive { message: String },
    Timeout { duration: chrono::Duration },
}

impl From<ipc::error::RequestError> for RequestError {
    fn from(err: ipc::error::RequestError) -> Self {
        match err {
            ipc::error::RequestError::Send(err) => RequestError::Send {
                message: err.to_string(),
            },
            ipc::error::RequestError::Receive(err) => RequestError::Receive {
                message: err.to_string(),
            },
            ipc::error::RequestError::Timeout(duration) => RequestError::Timeout {
                duration: chrono::Duration::from_std(duration).unwrap(),
            },
        }
    }
}

#[frb(dart_code = "
    @override
    String toString() => switch (this) {
        ReceiveError(:final message) => 'Failed to receive event: $message',
    };
")]
pub struct ReceiveError {
    pub message: String,
}

impl From<ipc::error::ReceiveError> for ReceiveError {
    fn from(err: ipc::error::ReceiveError) -> Self {
        ReceiveError {
            message: err.to_string(),
        }
    }
}

#[frb(dart_code = "
    @override
    String toString() => switch (this) {
        PersistError_Open(:final message) => 'Failed to open registry key: $message',
        PersistError_Set(:final message) => 'Failed to set registry key: $message',
        PersistError_Serialize(:final message) => 'Failed to serialize data: $message',
    };
")]
pub enum PersistError {
    Open { message: String },
    Set { message: String },
    Serialize { message: String },
}

impl From<ipc::error::PersistError> for PersistError {
    fn from(err: ipc::error::PersistError) -> Self {
        match err {
            ipc::error::PersistError::Open(err) => PersistError::Open {
                message: err.to_string(),
            },
            ipc::error::PersistError::Set(err) => PersistError::Set {
                message: err.to_string(),
            },
            ipc::error::PersistError::Serialize(err) => PersistError::Serialize {
                message: err.to_string(),
            },
        }
    }
}
