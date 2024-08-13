use std::{io, sync::Arc, time::Duration};

use log::error;
use serde::Serialize;
use tokio::{
    net::windows::named_pipe,
    sync::{broadcast, Notify, RwLock},
    task,
    time::timeout,
};
use tokio_stream::{Stream, StreamExt};

use crate::*;

// EOF byte used to separate messages
pub(crate) const EOF: u8 = 0x4;

/// Client for interacting with the Virtual Display Driver.
///
/// Connects via a named pipe to the driver.
///
/// You can send changes to the driver and receive continuous events from it.
///
/// It is save to clone this client. The connection is shared between all
/// copies.
#[derive(Debug)]
pub struct Client {
    shared: Arc<_Shared>,
    command_rx: broadcast::Receiver<Result<ClientCommand, error::ReceiveError>>,
}

#[derive(Debug)]
struct _Shared {
    client: named_pipe::NamedPipeClient,
    abort_receiver: Notify,
    receive_error: RwLock<Option<Arc<io::Error>>>,
}

impl Client {
    /// Connect to driver on pipe with default name.
    ///
    /// The default name is [DEFAULT_PIPE_NAME].
    ///
    /// This method is async because it requires a running tokio reactor.
    pub async fn connect() -> Result<Self, error::ConnectionError> {
        Self::connect_to(DEFAULT_PIPE_NAME).await
    }

    /// Connect to driver on pipe with specified name.
    ///
    /// `name` is ONLY the {name} portion of \\.\pipe\{name}.
    ///
    /// This method is async because it requires a running tokio reactor.
    pub async fn connect_to(name: &str) -> Result<Self, error::ConnectionError> {
        let client = named_pipe::ClientOptions::new()
            .read(true)
            .write(true)
            .pipe_mode(named_pipe::PipeMode::Byte)
            .open(format!(r"\\.\pipe\{name}"))?;

        let abort_receiver = Notify::new();

        let shared = Arc::new(_Shared {
            client,
            abort_receiver,
            receive_error: RwLock::new(None),
        });

        let (command_tx, command_rx) =
            broadcast::channel::<Result<ClientCommand, error::ReceiveError>>(10);

        {
            let shared = shared.clone();
            task::spawn(async move {
                let r = receive_command(&shared.client, &command_tx, &shared.abort_receiver).await;
                if let Err(e) = r {
                    let error = Arc::new(e);
                    shared.receive_error.write().await.replace(error.clone());
                    let _ = command_tx.send(Err(error::ReceiveError(error.clone())));
                }
            });
        }

        Ok(Self { shared, command_rx })
    }

    /// Send new state to the driver.
    pub async fn notify(&self, monitors: &[Monitor]) -> Result<(), error::SendError> {
        let command = DriverCommand::Notify(monitors.to_owned());

        send_command(&self.shared.client, &command).await?;
        Ok(())
    }

    /// Remove all monitors with the specified IDs.
    pub async fn remove(&self, ids: &[Id]) -> Result<(), error::SendError> {
        let command = DriverCommand::Remove(ids.to_owned());

        send_command(&self.shared.client, &command).await?;
        Ok(())
    }

    /// Remove all monitors.
    pub async fn remove_all(&self) -> Result<(), error::SendError> {
        let command = DriverCommand::RemoveAll;

        send_command(&self.shared.client, &command).await?;
        Ok(())
    }

    /// Request the current state of the driver.
    ///
    /// Returns [IpcError::Timeout] if the driver does not respond within 5
    /// seconds.
    pub async fn request_state(&self) -> Result<Vec<Monitor>, error::RequestError> {
        use broadcast::error::RecvError;

        let mut rx = self.command_rx.resubscribe();

        send_command(&self.shared.client, &RequestCommand::State).await?;

        let fut = async {
            loop {
                match rx.recv().await {
                    Ok(Ok(ClientCommand::Reply(ReplyCommand::State(monitors)))) => {
                        break Ok(monitors)
                    }
                    Ok(Err(e)) => break Err(error::RequestError::Receive(e.0.clone())),
                    Ok(_) => continue,
                    Err(RecvError::Lagged(_n)) => continue,
                    Err(RecvError::Closed) => match self.shared.receive_error.read().await.as_ref()
                    {
                        Some(e) => break Err(error::RequestError::Receive(e.clone())),
                        None => {
                            break Err(error::RequestError::Receive(Arc::new(io::Error::new(
                                io::ErrorKind::BrokenPipe,
                                "Pipe closed",
                            ))))
                        }
                    },
                }
            }
        };

        match timeout(Duration::from_secs(5), fut).await {
            Ok(result) => result,
            Err(_) => Err(error::RequestError::Timeout(Duration::from_secs(5))),
        }
    }

    /// Receive continuous events from the driver.
    ///
    /// Only new events after calling this method are received.
    ///
    /// May be called multiple times.
    ///
    /// Note: If multiple copies of this client exist, the receiver will only be
    /// closed after all copies are dropped.
    pub fn receive_events(&self) -> impl Stream<Item = Result<EventCommand, error::ReceiveError>> {
        use tokio_stream::wrappers::*;

        let stream = BroadcastStream::new(self.command_rx.resubscribe());

        stream.filter_map(|cmd| match cmd {
            Ok(Ok(ClientCommand::Event(event))) => Some(Ok(event)),
            Ok(Err(e)) => Some(Err(e)),
            Err(errors::BroadcastStreamRecvError::Lagged(_n)) => None, // TODO: Indicate lagged? (Maybe changing Item to Result<EventCommand, ...> is better?)
            _ => None,
        })
    }

    /// Write `monitors` to the registry for current user.
    ///
    /// Next time the driver is started, it will load this state from the
    /// registry. This might be after a reboot or a driver restart.
    pub fn persist(monitors: &[Monitor]) -> Result<(), error::PersistError> {
        use winreg::*;

        let hklm = RegKey::predef(enums::HKEY_CURRENT_USER);
        let key = r"SOFTWARE\VirtualDisplayDriver";

        let mut reg_key = hklm.open_subkey_with_flags(key, enums::KEY_WRITE);

        // if open failed, try to create key and subkey
        if reg_key.is_err() {
            reg_key = hklm.create_subkey(key).map(|(key, _)| key);

            if let Err(e) = reg_key {
                return Err(error::PersistError::Open(e));
            }
        }

        let reg_key = reg_key.unwrap();

        let data = serde_json::to_string(monitors)?;

        reg_key
            .set_value("data", &data)
            .map_err(error::PersistError::Set)?;

        Ok(())
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            shared: self.shared.clone(),
            command_rx: self.command_rx.resubscribe(),
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if Arc::strong_count(&self.shared) == 2 {
            self.shared.abort_receiver.notify_waiters();
        }
    }
}

async fn send_command(
    client: &named_pipe::NamedPipeClient,
    command: &impl Serialize,
) -> Result<(), error::SendCommandError> {
    // Create a vector with the full message, then send it as a single
    // write. This is required because the pipe is in message mode.
    let mut message = serde_json::to_vec(command)?;
    message.push(EOF);

    // write to pipe without needing to block or split it

    let mut written = 0;
    loop {
        // wait for pipe to be writable
        client.writable().await?;

        match client.try_write(&message[written..]) {
            // we wrote less than the entire size
            Ok(n) if written + n < message.len() => {
                written += n;
                continue;
            }

            // write succeeded
            Ok(n) if written + n >= message.len() => {
                break;
            }

            // nothing wrote, retry again
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }

            // actual error
            Err(e) => {
                return Err(e.into());
            }

            _ => unreachable!(),
        }
    }

    Ok(())
}

// receive all commands and send them back to the receiver
async fn receive_command(
    client: &named_pipe::NamedPipeClient,
    tx: &broadcast::Sender<Result<ClientCommand, error::ReceiveError>>,
    abort: &Notify,
) -> Result<(), io::Error> {
    let mut buf = vec![0; 4096];
    let mut recv_buf = Vec::with_capacity(4096);

    loop {
        // wait for client to be readable
        tokio::select! {
            r = client.readable() => r?,
            _ = abort.notified() => return Ok(()),
        }

        match client.try_read(&mut buf) {
            Ok(0) => return Err(io::Error::last_os_error()),
            Ok(n) => recv_buf.extend(&buf[..n]),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(e) => return Err(e),
        }

        let eof_iter =
            recv_buf.iter().enumerate().filter_map(
                |(i, &byte)| {
                    if byte == EOF {
                        Some(i)
                    } else {
                        None
                    }
                },
            );

        let mut offset = 0;
        for pos in eof_iter {
            let data = &recv_buf[offset..pos];
            offset = pos + 1;

            let Ok(command) = serde_json::from_slice::<ClientCommand>(data) else {
                continue;
            };

            if tx.send(Ok(command)).is_err() {
                // Client closed, abort
                return Ok(());
            }
        }

        // drain all processed messages
        recv_buf.drain(..offset);
    }
}

pub mod error {
    use super::*;
    use thiserror::Error;

    /// Error returned from [Client::connect] and [Client::connect_to].
    #[derive(Debug, Error)]
    pub enum ConnectionError {
        #[error("Failed to open pipe: {0}")]
        Failed(#[from] io::Error),
    }

    /// Error returned from [send_command]
    #[derive(Debug, Error)]
    pub(super) enum SendCommandError {
        #[error("Failed to encode message: {0}")]
        Encode(#[from] serde_json::Error),
        #[error("Failed to send message: {0}")]
        PipeBroken(#[from] io::Error),
    }

    /// Error returned from [Client::notify], [Client::remove] and
    /// [Client::remove_all].
    #[derive(Debug, Error)]
    pub enum SendError {
        #[error("Failed to send message: {0}")]
        PipeBroken(#[from] io::Error),
    }

    /// Error returned from [Client::request_state].
    #[derive(Debug, Error)]
    pub enum RequestError {
        #[error("Failed to send message (pipe broken): {0}")]
        Send(io::Error),
        #[error("Failed to receive message (pipe broken): {0}")]
        Receive(Arc<io::Error>),
        #[error("Did not get a response in time ({0:?})")]
        Timeout(Duration),
    }

    /// Error returned from [Client::receive_events].
    #[derive(Debug, Error, Clone)]
    #[error("Failed to receive event: {0}")]
    pub struct ReceiveError(#[from] pub Arc<io::Error>);

    /// Error returned from [Client::persist].
    #[derive(Debug, Error)]
    pub enum PersistError {
        #[error("Failed to open registry key: {0}")]
        Open(io::Error),
        #[error("Failed to set registry value: {0}")]
        Set(io::Error),
        #[error("Failed to serialize monitors: {0}")]
        Serialize(#[from] serde_json::Error),
    }

    impl From<SendCommandError> for SendError {
        fn from(e: SendCommandError) -> Self {
            match e {
                SendCommandError::PipeBroken(e) => Self::PipeBroken(e),
                SendCommandError::Encode(e) => unreachable!("{:?}", e),
            }
        }
    }

    impl From<SendCommandError> for RequestError {
        fn from(e: SendCommandError) -> Self {
            match e {
                SendCommandError::PipeBroken(e) => Self::Send(e),
                SendCommandError::Encode(e) => unreachable!("{:?}", e),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use tokio::time::sleep;

    use super::*;
    use crate::mock::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn receiver_stops_when_client_closed() {
        const PIPE_NAME: &str = "virtualdisplaydriver-test-receiver_stops_when_client_closed";

        let mut server = MockServer::new(PIPE_NAME).await;

        let client1 = Client::connect_to(PIPE_NAME)
            .await
            .expect("Failed to connect to pipe");
        let stream1 = client1.receive_events();

        let client2 = client1.clone();
        let stream2 = client2.receive_events();

        client1.notify(&[]).await.expect("Failed to notify");
        server.pump().await;

        sleep(Duration::from_millis(50)).await;

        drop(client1);

        client2.notify(&[]).await.expect("Failed to notify");
        server.pump().await;

        sleep(Duration::from_millis(50)).await;

        drop(client2);

        let events1: Vec<_> = stream1.collect().await;
        let events2: Vec<_> = stream2.collect().await;

        assert_eq!(events1.len(), 2, "{:?}", events1);
        assert_eq!(events2.len(), 2, "{:?}", events2);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn receiver_stops_when_server_closed() {
        const PIPE_NAME: &str = "virtualdisplaydriver-test-receiver_stops_when_server_closed";

        let server = MockServer::new(PIPE_NAME).await;

        let client = Client::connect_to(PIPE_NAME)
            .await
            .expect("Failed to connect to pipe");

        let stream = client.receive_events();

        sleep(Duration::from_millis(50)).await;

        drop(server);

        let events: Vec<_> = stream.collect().await;

        println!("{:?}", events);

        assert!(
            matches!(events[..], [Err(error::ReceiveError(ref e))] if e.kind() == io::ErrorKind::BrokenPipe)
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn general_test_1() {
        const PIPE_NAME: &str = "virtualdisplaydriver-test-general_test_1";

        let mut server = MockServer::new(PIPE_NAME).await;

        let client = Client::connect_to(PIPE_NAME)
            .await
            .expect("Failed to connect to pipe");

        // Get receiver stream

        let stream = client.receive_events();

        // Check request_state

        server.check_next(|cmd| {
            assert!(matches!(cmd, ServerCommand::Request(RequestCommand::State)));
        });

        let (state, _) = tokio::join!(client.request_state(), server.pump());

        let state = state.expect("Failed to request state");
        assert!(state.is_empty());

        // Check notify

        let mons1 = [Monitor {
            id: 0,
            enabled: true,
            name: Some("test".to_string()),
            modes: vec![Mode {
                width: 1920,
                height: 1080,
                refresh_rates: vec![60],
            }],
        }];

        let fut = client.notify(&mons1);

        tokio::join!(fut, server.pump())
            .0
            .expect("Failed to notify");

        assert_eq!(server.state(), &mons1);

        // Check request_state

        let (state, _) = tokio::join!(client.request_state(), server.pump());

        let state = state.expect("Failed to request state");
        assert_eq!(&state, &mons1);

        // Check notify multiple

        let mons2 = [
            Monitor {
                id: 0,
                enabled: false,
                name: Some("test1".to_string()),
                modes: vec![Mode {
                    width: 100,
                    height: 200,
                    refresh_rates: vec![80, 90],
                }],
            },
            Monitor {
                id: 1,
                enabled: true,
                name: Some("test2".to_string()),
                modes: vec![Mode {
                    width: 300,
                    height: 400,
                    refresh_rates: vec![50],
                }],
            },
        ];

        tokio::join!(client.notify(&mons2), server.pump())
            .0
            .expect("Failed to notify");

        assert_eq!(server.state(), &mons2);

        // Give some time for the server to send the last event
        sleep(Duration::from_millis(50)).await;

        let stream2 = client.receive_events();

        // Check remove

        tokio::join!(client.remove(&[0]), server.pump())
            .0
            .expect("Failed to remove");

        assert_eq!(server.state().len(), 1);
        assert_eq!(server.state()[0].id, 1);

        // Check remove all

        tokio::join!(client.remove_all(), server.pump())
            .0
            .expect("Failed to remove all");

        assert!(server.state().is_empty());

        // Check streams

        // Give some time for the server to send the last event
        sleep(Duration::from_millis(50)).await;

        drop(client);

        let events: Vec<_> = stream.collect().await;

        assert!(matches!(events[..], [
                Ok(EventCommand::Changed(ref e1)),
                Ok(EventCommand::Changed(ref e2)),
                Ok(EventCommand::Changed(ref e3)),
                Ok(EventCommand::Changed(ref e4)),
            ] if *e1 == mons1
                && *e2 == mons2
                && *e3 == mons2[1..]
                && e4.is_empty()
        ));

        let events: Vec<_> = stream2.collect().await;

        assert!(matches!(events[..], [
                Ok(EventCommand::Changed(ref e1)),
                Ok(EventCommand::Changed(ref e2)),
            ] if  *e1 == mons2[1..]
                && e2.is_empty()
        ));
    }
}
