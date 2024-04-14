use std::{io, sync::Arc, time::Duration};

use log::error;
use serde::Serialize;
use tokio::{
    net::windows::named_pipe,
    sync::{broadcast, Notify},
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
    client: Arc<named_pipe::NamedPipeClient>,
    command_rx: broadcast::Receiver<ClientCommand>,
    abort_receiver: Arc<Notify>,
}

impl Client {
    /// Connect to driver on pipe with default name.
    ///
    /// The default name is [DEFAULT_PIPE_NAME].
    pub fn connect() -> Result<Self> {
        Self::connect_to(DEFAULT_PIPE_NAME)
    }

    /// Connect to driver on pipe with specified name.
    ///
    /// `name` is ONLY the {name} portion of \\.\pipe\{name}.
    pub fn connect_to(name: &str) -> Result<Self> {
        let client = named_pipe::ClientOptions::new()
            .read(true)
            .write(true)
            .pipe_mode(named_pipe::PipeMode::Byte)
            .open(format!(r"\\.\pipe\{name}"))
            .map_err(IpcError::ConnectionFailed)?;
        let client = Arc::new(client);

        let abort_receiver = Arc::new(Notify::new());

        let (command_tx, command_rx) = broadcast::channel::<ClientCommand>(10);

        {
            let client = client.clone();
            let abort_receiver = abort_receiver.clone();
            task::spawn(async move {
                let r = receive_command(&client, command_tx, &abort_receiver).await;
                if let Err(e) = r {
                    println!("TODO: Handle error: {:?}", e);
                }
            });
        }

        Ok(Self {
            client,
            command_rx,
            abort_receiver,
        })
    }

    /// Send new state to the driver.
    pub async fn notify(&self, monitors: &[Monitor]) -> Result<()> {
        let command = DriverCommand::Notify(monitors.to_owned());

        send_command(&self.client, &command).await
    }

    /// Remove all monitors with the specified IDs.
    pub async fn remove(&self, ids: &[Id]) -> Result<()> {
        let command = DriverCommand::Remove(ids.to_owned());

        send_command(&self.client, &command).await
    }

    /// Remove all monitors.
    pub async fn remove_all(&self) -> Result<()> {
        let command = DriverCommand::RemoveAll;

        send_command(&self.client, &command).await
    }

    /// Request the current state of the driver.
    ///
    /// Returns [IpcError::Timeout] if the driver does not respond within 5
    /// seconds.
    pub async fn request_state(&self) -> Result<Vec<Monitor>> {
        let mut rx = self.command_rx.resubscribe();

        send_command(&self.client, &RequestCommand::State).await?;

        let fut = async {
            loop {
                match rx.recv().await {
                    Ok(ClientCommand::Reply(ReplyCommand::State(monitors))) => break Ok(monitors),
                    Ok(_) => continue,
                    Err(_) => break Err(IpcError::Receive),
                }
            }
        };

        match timeout(Duration::from_secs(5), fut).await {
            Ok(result) => result,
            Err(_) => Err(IpcError::Timeout),
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
    pub fn receive_events(&self) -> impl Stream<Item = EventCommand> {
        use tokio_stream::wrappers::*;

        let stream = BroadcastStream::new(self.command_rx.resubscribe());

        stream.filter_map(|cmd| match cmd {
            Ok(ClientCommand::Event(event)) => Some(event),
            Err(errors::BroadcastStreamRecvError::Lagged(_n)) => None, // TODO: Indicate lagged? (Maybe changing Item to Result<EventCommand, ...> is better?)
            _ => None,
        })
    }

    /// Write `monitors` to the registry for current user.
    ///
    /// Next time the driver is started, it will load this state from the
    /// registry. This might be after a reboot or a driver restart.
    pub fn persist(monitors: &[Monitor]) -> Result<()> {
        use winreg::*;

        let hklm = RegKey::predef(enums::HKEY_CURRENT_USER);
        let key = r"SOFTWARE\VirtualDisplayDriver";

        let mut reg_key = hklm.open_subkey_with_flags(key, enums::KEY_WRITE);

        // if open failed, try to create key and subkey
        if let Err(e) = reg_key {
            error!("Failed opening {key}: {e:?}");
            reg_key = hklm.create_subkey(key).map(|(key, _)| key);

            if let Err(e) = reg_key {
                error!("Failed creating {key}: {e:?}");
                return Err(e)?;
            }
        }

        let reg_key = reg_key.unwrap();

        let data = serde_json::to_string(monitors)?;

        reg_key.set_value("data", &data)?;

        Ok(())
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            command_rx: self.command_rx.resubscribe(),
            abort_receiver: self.abort_receiver.clone(),
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if Arc::strong_count(&self.abort_receiver) == 2 {
            self.abort_receiver.notify_waiters();
        }
    }
}

async fn send_command(
    client: &named_pipe::NamedPipeClient,
    command: &impl Serialize,
) -> Result<()> {
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
    tx: broadcast::Sender<ClientCommand>,
    abort: &Notify,
) -> Result<()> {
    let mut buf = vec![0; 4096];
    let mut recv_buf = Vec::with_capacity(4096);

    loop {
        // wait for client to be readable
        tokio::select! {
            r = client.readable() => r?,
            _ = abort.notified() => return Ok(()),
        }

        match client.try_read(&mut buf) {
            Ok(0) => return Err(IpcError::Io(io::Error::last_os_error())),

            Ok(n) => recv_buf.extend(&buf[..n]),

            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }

            Err(e) => {
                return Err(e.into());
            }
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

        let mut bidx = 0;
        for pos in eof_iter {
            let data = &recv_buf[bidx..pos];
            bidx = pos + 1;

            let Ok(command) = serde_json::from_slice::<ClientCommand>(data) else {
                continue;
            };

            if tx.send(command).is_err() {
                return Err(IpcError::SendFailed);
            }
        }

        // drain all processed messages
        recv_buf.drain(..bidx);
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

        let mut server = MockServer::new(PIPE_NAME);

        let client1 = Client::connect_to(PIPE_NAME).expect("Failed to connect to pipe");
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

        let server = MockServer::new(PIPE_NAME);

        let client = Client::connect_to(PIPE_NAME).expect("Failed to connect to pipe");

        let stream = client.receive_events();

        sleep(Duration::from_millis(50)).await;

        drop(server);

        let events: Vec<_> = stream.collect().await;

        assert!(events.is_empty());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn general_test_1() {
        const PIPE_NAME: &str = "virtualdisplaydriver-test-general_test_1";

        let mut server = MockServer::new(PIPE_NAME);

        let client = Client::connect_to(PIPE_NAME).expect("Failed to connect to pipe");

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
                EventCommand::Changed(ref e1),
                EventCommand::Changed(ref e2),
                EventCommand::Changed(ref e3),
                EventCommand::Changed(ref e4),
            ] if *e1 == mons1
                && *e2 == mons2
                && *e3 == mons2[1..]
                && e4.is_empty()
        ));

        let events: Vec<_> = stream2.collect().await;

        assert!(matches!(events[..], [
                EventCommand::Changed(ref e1),
                EventCommand::Changed(ref e2),
            ] if  *e1 == mons2[1..]
                && e2.is_empty()
        ));
    }
}
