use std::{any::Any, panic, thread};

use tokio::sync::{mpsc, oneshot};
use tokio_stream::StreamExt;

use super::RUNTIME;
use crate::{client::error, Client as AsyncClient, EventCommand, Id, Monitor};

/// Client for interacting with the Virtual Display Driver.
///
/// Connects via a named pipe to the driver.
///
/// You can send changes to the driver and receive continuous events from it.
///
/// It is save to clone this client. The connection is shared between all
/// copies.
///
/// This is a synchronous version of [crate::Client]. It uses its own tokio
/// runtime. This runtime is configured with a single worker thread.
#[derive(Debug, Clone)]
pub struct Client(AsyncClient);

impl Client {
    /// Connect to driver on pipe with default name.
    ///
    /// The default name is [crate::DEFAULT_PIPE_NAME].
    pub fn connect() -> Result<Self, error::ConnectionError> {
        let client = RUNTIME.block_on(async { AsyncClient::connect() })?;
        Ok(Self(client))
    }

    /// Connect to driver on pipe with specified name.
    ///
    /// `name` is ONLY the {name} portion of \\.\pipe\{name}.
    pub fn connect_to(name: &str) -> Result<Self, error::ConnectionError> {
        let client = RUNTIME.block_on(async { AsyncClient::connect_to(name) })?;
        Ok(Self(client))
    }

    /// Send new state to the driver.
    pub fn notify(&self, monitors: &[Monitor]) -> Result<(), error::SendError> {
        RUNTIME.block_on(self.0.notify(monitors))
    }

    /// Remove all monitors with the specified IDs.
    pub fn remove(&self, ids: &[Id]) -> Result<(), error::SendError> {
        RUNTIME.block_on(self.0.remove(ids))
    }

    /// Remove all monitors.
    pub fn remove_all(&self) -> Result<(), error::SendError> {
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
    /// Note: The callback should return as soon as possible. It is called from
    /// the library's tokio runtime and blocks all other operations. In
    /// consequence, all other library events will be delayed until the callback
    /// returns.
    ///
    /// Note: If multiple copies of this client exist, the receiver will only be
    /// closed after all copies are dropped.
    pub fn add_event_receiver(
        &self,
        cb: impl FnMut(EventCommand) + Send + panic::UnwindSafe + 'static,
    ) -> EventsSubscription {
        let stream = self.0.receive_events();
        EventsSubscription::start_subscriber(cb, stream)
    }

    /// Request the current state of the driver.
    ///
    /// Returns [IpcError::Timeout] if the driver does not respond within 5
    /// seconds.
    pub fn request_state(&self) -> Result<Vec<Monitor>, error::RequestError> {
        RUNTIME.block_on(self.0.request_state())
    }

    /// Write `monitors` to the registry for current user.
    ///
    /// Next time the driver is started, it will load this state from the
    /// registry. This might be after a reboot or a driver restart.
    pub fn persist(monitors: &[Monitor]) -> Result<(), error::PersistError> {
        AsyncClient::persist(monitors)
    }
}

pub struct EventsSubscription {
    pub(crate) abort_tx: mpsc::Sender<()>,
    result_rx: Option<oneshot::Receiver<Box<dyn Any + Send>>>,
}

impl EventsSubscription {
    pub(crate) fn start_subscriber(
        mut cb: impl FnMut(EventCommand) + Send + panic::UnwindSafe + 'static,
        mut stream: impl tokio_stream::Stream<Item = EventCommand> + Unpin + Send + 'static,
    ) -> Self {
        let (abort_tx, mut abort_rx) = mpsc::channel(1);
        let (result_tx, result_rx) = oneshot::channel();

        RUNTIME.spawn(async move {
            while let Some(event) = if abort_rx.is_closed() {
                stream.next().await
            } else {
                tokio::select! {
                    event = stream.next() => event,
                    _ = abort_rx.recv() => None,
                }
            } {
                let mut cb = panic::AssertUnwindSafe(&mut cb);
                let res = panic::catch_unwind(move || {
                    cb(event);
                });
                if let Err(e) = res {
                    if let Err(e) = result_tx.send(e) {
                        log::error!(
                            "Event receiver panicked, but subscription is already dropped: {:?}",
                            e
                        );
                    };
                    break;
                }
            }
        });

        Self {
            abort_tx,
            result_rx: Some(result_rx),
        }
    }

    /// Cancel the subscription.
    ///
    /// Returns `Ok(true)` if the subscription was not already cancelled.
    ///
    /// Returns `Ok(false)` if the subscription was already cancelled.
    ///
    /// Returns `Err(e)` if the callback panicked. Any subsequent calls will
    /// return `Ok(false)`.
    ///
    /// Will return immediately. A panic will not be caught if the callback
    /// panics after calling this method.
    pub fn cancel(&mut self) -> thread::Result<bool> {
        let Some(ref mut rx) = self.result_rx else {
            // Already cancelled by `cancel_async`
            return Ok(false);
        };

        match rx.try_recv() {
            Err(oneshot::error::TryRecvError::Empty) => {
                // Returns error when either buffer is full (buffer size = 1), or
                // receiver is closed <=> already cancelled
                Ok(self.abort_tx.try_send(()).is_ok())
            }
            Err(oneshot::error::TryRecvError::Closed) => Ok(false),
            Ok(e) => Err(e),
        }
    }

    /// Cancel the subscription.
    ///
    /// Returns `Ok(true)` if the subscription was not already cancelled.
    ///
    /// Returns `Ok(false)` if the subscription was already cancelled.
    ///
    /// Returns `Err(e)` if the callback panicked. Any subsequent calls will
    /// return `Ok(false)`.
    ///
    /// This method blocks until the callback either returns or panics.
    pub fn cancel_blocking(&mut self) -> thread::Result<bool> {
        RUNTIME.block_on(self.cancel_async())
    }

    /// Cancel the subscription.
    ///
    /// Returns `Ok(true)` if the subscription was not already cancelled.
    ///
    /// Returns `Ok(false)` if the subscription was already cancelled.
    ///
    /// Returns `Err(e)` if the callback panicked. Any subsequent calls will
    /// return `Ok(false)`.
    ///
    /// This method waits until the callback either returns or panics.
    pub async fn cancel_async(&mut self) -> thread::Result<bool> {
        // Returns error when either buffer is full (buffer size = 1), or
        // receiver is closed <=> already cancelled
        let Some(rx) = self.result_rx.take() else {
            // Already cancelled by `cancel_async`
            return Ok(false);
        };

        let res = self.abort_tx.try_send(()).is_ok();
        match rx.await {
            Err(_) => Ok(res),
            Ok(e) => Err(e),
        }
    }
}

impl Drop for EventsSubscription {
    fn drop(&mut self) {
        let Some(ref mut rx) = self.result_rx else {
            return;
        };
        if let Err(e) = rx.try_recv() {
            log::error!(
                "Event receiver panicked, but subscription is dropped without canceling: {:?}",
                e
            );
        }
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
    fn event_receiver_not_canceled_after_drop() {
        const PIPE_NAME: &str = "virtualdisplaydriver-sync-event_receiver_not_canceled_after_drop";

        let mut server = RUNTIME.block_on(async { MockServer::new(PIPE_NAME) });

        let client = Client::connect_to(PIPE_NAME).unwrap();

        let call_count = Arc::new(Mutex::new(0));

        let sub = client.add_event_receiver({
            let call_count = call_count.clone();
            move |_| {
                *call_count.lock().unwrap() += 1;
            }
        });

        drop(sub);

        client.notify(&[]).unwrap();
        RUNTIME.block_on(server.pump());

        // Give time for the callback to be run
        sleep(std::time::Duration::from_millis(100));

        assert_eq!(*call_count.lock().unwrap(), 1);
    }

    #[test]
    fn catch_unwind_when_receiver_panics() {
        const PIPE_NAME: &str = "virtualdisplaydriver-sync-catch_unwind_when_receiver_panics";

        let mut server = RUNTIME.block_on(async { MockServer::new(PIPE_NAME) });

        let client = Client::connect_to(PIPE_NAME).unwrap();

        let mut sub1 = client.add_event_receiver(move |_| {
            panic!("Panic1 in callback");
        });
        let mut sub2 = client.add_event_receiver(move |_| {
            panic!("Panic2 in callback");
        });

        client.notify(&[]).unwrap();
        RUNTIME.block_on(server.pump());

        // Give time for the callback to be run
        sleep(std::time::Duration::from_millis(100));

        sub1.cancel_blocking().expect_err("Callback should panic");
        assert!(!sub1
            .cancel_blocking()
            .expect("Error should already be handled"));

        sub2.cancel().expect_err("Callback should panic");
        assert!(!sub2.cancel().expect("Error should already be handled"));
    }

    #[test]
    fn event_receiver() {
        const PIPE_NAME: &str = "virtualdisplaydriver-sync-event_receiver";

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
        sleep(std::time::Duration::from_millis(100));

        assert!(sub.cancel().expect("Callback should not panic"));
        assert!(!sub.cancel().expect("Callback should not panic"));
        assert!(!sub.cancel().expect("Callback should not panic"));

        client.notify(&[]).unwrap();
        RUNTIME.block_on(server.pump());
        sleep(std::time::Duration::from_millis(100));

        assert!(matches!(
            events.lock().unwrap().as_slice(),
            [EventCommand::Changed(mons)] if mons.is_empty()
        ))
    }

    #[test]
    fn event_receiver_cancel_from_cb() {
        const PIPE_NAME: &str = "virtualdisplaydriver-sync-event_receiver_cancel_from_cb";

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
                    shared_sub
                        .lock()
                        .unwrap()
                        .as_mut()
                        .unwrap()
                        .cancel()
                        .expect("Callback should not panic"),
                    "Should not be cancelled by now"
                );
                *shared_flag.lock().unwrap() = true;
            }
        });

        *shared_sub.lock().unwrap() = Some(sub);

        client.notify(&[]).unwrap();
        RUNTIME.block_on(server.pump());
        sleep(std::time::Duration::from_millis(100));

        assert!(
            !shared_sub
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .cancel()
                .expect("Callback should not panic"),
            "Should already be cancelled"
        );
        assert!(*shared_flag.lock().unwrap(), "Callback was not run");
    }
}
