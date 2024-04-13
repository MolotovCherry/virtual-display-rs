use tokio_stream::StreamExt;

use super::RUNTIME;
use crate::{Client as AsyncClient, EventCommand, Id, Monitor, Result};

/// A thin api client over the driver api with all the essential api.
/// Does the bare minimum required. Does not track state
#[derive(Debug, Clone)]
pub struct Client(AsyncClient);

impl Client {
    /// connect to pipe virtualdisplaydriver
    pub fn connect() -> Result<Self> {
        let client = AsyncClient::connect()?;
        Ok(Self(client))
    }

    // choose which pipe name you connect to
    // pipe name is ONLY the name, only the {name} portion of \\.\pipe\{name}
    pub fn connect_to(name: &str) -> Result<Self> {
        let client = AsyncClient::connect_to(name)?;
        Ok(Self(client))
    }

    /// Notifies driver of changes (additions/updates/removals)
    pub fn notify(&self, monitors: &[Monitor]) -> Result<()> {
        RUNTIME.block_on(self.0.notify(monitors))
    }

    /// Remove specific monitors by id
    pub fn remove(&self, ids: &[Id]) -> Result<()> {
        RUNTIME.block_on(self.0.remove(ids))
    }

    /// Remove all monitors
    pub fn remove_all(&self) -> Result<()> {
        RUNTIME.block_on(self.0.remove_all())
    }

    /// Receive an event. Only new events after calling this are received
    pub fn receive_event(&mut self) -> EventCommand {
        RUNTIME.block_on(async {
            self.0
                .receive_events()
                .next()
                .await
                .expect("Stream should never finish")
        })
    }

    pub fn add_event_receiver(&mut self, cb: impl Fn(EventCommand) -> bool + Send + 'static) {
        let mut stream = self.0.receive_events();

        RUNTIME.spawn(async move {
            while let Some(event) = stream.next().await {
                if !cb(event) {
                    break;
                }
            }
        });
    }

    /// Request state update
    /// use `receive()` to get the reply
    pub fn request_state(&self) -> Result<Vec<Monitor>> {
        RUNTIME.block_on(self.0.request_state())
    }

    /// Persist changes to registry for current user
    pub fn persist(monitors: &[Monitor]) -> Result<()> {
        AsyncClient::persist(monitors)
    }
}
