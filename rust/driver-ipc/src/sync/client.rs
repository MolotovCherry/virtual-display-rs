use tokio::sync::mpsc;
use tokio_stream::StreamExt;

use super::RUNTIME;
use crate::{Client as AsyncClient, EventCommand, Id, Monitor, Result};

/// Client for interacting with the Virtual Display Driver.
///
/// Connects via a named pipe to the driver.
///
/// You can send changes to the driver and receive continuous events from it.
///
/// This is a synchronous version of [crate::Client]. It uses its own tokio
/// runtime. This runtime is configured with a single worker thread.
#[derive(Debug, Clone)]
pub struct Client(AsyncClient);

impl Client {
    /// Connect to driver on pipe with default name.
    ///
    /// The default name is [crate::DEFAULT_PIPE_NAME].
    pub fn connect() -> Result<Self> {
        let client = RUNTIME.block_on(async { AsyncClient::connect() })?;
        Ok(Self(client))
    }

    /// Connect to driver on pipe with specified name.
    ///
    /// `name` is ONLY the {name} portion of \\.\pipe\{name}.
    pub fn connect_to(name: &str) -> Result<Self> {
        let client = RUNTIME.block_on(async { AsyncClient::connect_to(name) })?;
        Ok(Self(client))
    }

    /// Send new state to the driver.
    pub fn notify(&self, monitors: &[Monitor]) -> Result<()> {
        RUNTIME.block_on(self.0.notify(monitors))
    }

    /// Remove all monitors with the specified IDs.
    pub fn remove(&self, ids: &[Id]) -> Result<()> {
        RUNTIME.block_on(self.0.remove(ids))
    }

    /// Remove all monitors.
    pub fn remove_all(&self) -> Result<()> {
        RUNTIME.block_on(self.0.remove_all())
    }

    /// Block and receive the next driver event.
    ///
    /// Only new events after calling this method will be received.
    pub fn receive_event(&mut self) -> EventCommand {
        RUNTIME.block_on(async {
            self.0
                .receive_events()
                .next()
                .await
                .expect("Stream should never finish")
        })
    }

    /// Add an event receiver to receive continuous events from the driver.
    ///
    /// Returns an object that can be used to cancel the subscription.
    ///
    /// The callback should return as soon as possible. It is called from the
    /// library's tokio runtime and blocks all other operations. In consequence,
    /// all other library events will be delayed until the callback returns.
    pub fn add_event_receiver(
        &self,
        cb: impl FnMut(EventCommand) + Send + 'static,
    ) -> EventsSubscription {
        let stream = self.0.receive_events();
        EventsSubscription::start_subscriber(cb, stream)
    }

    /// Request the current state of the driver.
    ///
    /// Returns [IpcError::Timeout] if the driver does not respond within 5
    /// seconds.
    pub fn request_state(&self) -> Result<Vec<Monitor>> {
        RUNTIME.block_on(self.0.request_state())
    }

    /// Write `monitors` to the registry for current user.
    ///
    /// Next time the driver is started, it will load this state from the
    /// registry. This might be after a reboot or a driver restart.
    pub fn persist(monitors: &[Monitor]) -> Result<()> {
        AsyncClient::persist(monitors)
    }
}

pub struct EventsSubscription {
    pub(crate) abort_tx: mpsc::Sender<()>,
}

impl EventsSubscription {
    pub(crate) fn start_subscriber(
        mut cb: impl FnMut(EventCommand) + Send + 'static,
        mut stream: impl tokio_stream::Stream<Item = EventCommand> + Unpin + Send + 'static,
    ) -> Self {
        let (abort_tx, mut abort_rx) = mpsc::channel(1);

        RUNTIME.spawn(async move {
            while let Some(event) = tokio::select! {
                event = stream.next() => event,
                _ = abort_rx.recv() => None,
            } {
                cb(event);
            }
        });

        Self { abort_tx }
    }

    pub fn cancel(&mut self) -> bool {
        // Returns error when either buffer is full (buffer size = 1), or
        // receiver is closed <=> already cancelled
        self.abort_tx.try_send(()).is_ok()
    }
}

#[cfg(test)]
mod test {
    use std::{
        sync::{Arc, Mutex},
        thread::sleep,
    };

    use super::*;
    use crate::mock::*;

    #[test]
    fn event_receiver() {
        const PIPE_NAME: &str = "virtualdisplaydriver-sync-test1";

        let mut server = RUNTIME.block_on(async { MockServer::new(PIPE_NAME) });

        let client = Client::connect_to(PIPE_NAME).unwrap();

        let events = Arc::new(Mutex::new(vec![]));

        let mut sub = client.add_event_receiver({
            let events = events.clone();
            move |event| {
                events.lock().unwrap().push(event);
            }
        });

        client.notify(&[]).unwrap();
        RUNTIME.block_on(server.pump());
        sleep(std::time::Duration::from_millis(50));

        assert!(sub.cancel());
        assert!(!sub.cancel());
        assert!(!sub.cancel());

        client.notify(&[]).unwrap();
        RUNTIME.block_on(server.pump());
        sleep(std::time::Duration::from_millis(50));

        assert!(matches!(
            events.lock().unwrap().as_slice(),
            [EventCommand::Changed(mons)] if mons.is_empty()
        ))
    }

    #[test]
    fn event_receiver_cancel_from_cb() {
        const PIPE_NAME: &str = "virtualdisplaydriver-sync-test2";

        let mut server = RUNTIME.block_on(async { MockServer::new(PIPE_NAME) });

        let client = Client::connect_to(PIPE_NAME).unwrap();

        let shared_sub = Arc::new(Mutex::new(None::<EventsSubscription>));
        let shared_flag = Arc::new(Mutex::new(false));

        let sub = client.add_event_receiver({
            let shared_sub = shared_sub.clone();
            let shared_flag = shared_flag.clone();
            move |event| {
                assert!(
                    matches!(event, EventCommand::Changed(mons) if mons.is_empty()),
                    "Wrong event received"
                );
                assert!(
                    shared_sub.lock().unwrap().as_mut().unwrap().cancel(),
                    "Should not be cancelled by now"
                );
                *shared_flag.lock().unwrap() = true;
            }
        });

        *shared_sub.lock().unwrap() = Some(sub);

        client.notify(&[]).unwrap();
        RUNTIME.block_on(server.pump());
        sleep(std::time::Duration::from_millis(50));

        assert!(
            !shared_sub.lock().unwrap().as_mut().unwrap().cancel(),
            "Should already be cancelled"
        );
        assert!(*shared_flag.lock().unwrap(), "Callback was not run");
    }
}
