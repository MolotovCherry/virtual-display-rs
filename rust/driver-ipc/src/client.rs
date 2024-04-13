use std::{convert::Infallible, io, sync::Arc, time::Duration};

use log::error;
use serde::Serialize;
use tokio::{net::windows::named_pipe, sync::broadcast, task, time::timeout};
use tokio_stream::{Stream, StreamExt};

use crate::*;

// EOF byte used to separate messages
const EOF: u8 = 0x4;

/// A thin api client over the driver api with all the essential api.
/// Does the bare minimum required. Does not track state
#[derive(Debug, Clone)]
pub struct Client {
    client: Arc<named_pipe::NamedPipeClient>,
    command_tx: broadcast::Sender<ClientCommand>,
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

        let (command_tx, _command_rx) = broadcast::channel::<ClientCommand>(10);

        {
            let client = client.clone();
            let command_tx = command_tx.clone();
            task::spawn(async move { receive_command(&client, command_tx).await });
        }

        Ok(Self { client, command_tx })
    }

    /// Notifies driver of changes (additions/updates/removals).
    pub async fn notify(&self, monitors: &[Monitor]) -> Result<()> {
        let command = DriverCommand::Notify(monitors.to_owned());

        send_command(&self.client, &command).await
    }

    /// Remove specific monitors by id.
    pub async fn remove(&self, ids: &[Id]) -> Result<()> {
        let command = DriverCommand::Remove(ids.to_owned());

        send_command(&self.client, &command).await
    }

    /// Remove all monitors.
    pub async fn remove_all(&self) -> Result<()> {
        let command = DriverCommand::RemoveAll;

        send_command(&self.client, &command).await
    }

    pub async fn request_state(&self) -> Result<Vec<Monitor>> {
        let mut rx = self.command_tx.subscribe();

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

    /// Receive events from the driver.
    ///
    /// Only new events after calling this method are received.
    ///
    /// May be called multiple times.
    pub fn receive_events(&self) -> impl Stream<Item = EventCommand> {
        use tokio_stream::wrappers::*;

        let stream = BroadcastStream::new(self.command_tx.subscribe());

        stream.filter_map(|cmd| match cmd {
            Ok(ClientCommand::Event(event)) => Some(event),
            Err(errors::BroadcastStreamRecvError::Lagged(_n)) => None, // TODO: Indicate lagged? (Maybe changing Item to Result<EventCommand, ...> is better?)
            _ => None,
        })
    }

    /// Persist changes to registry for current user
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
) -> Result<Infallible> {
    let mut buf = vec![0; 4096];
    let mut recv_buf = Vec::with_capacity(4096);

    loop {
        // wait for client to be readable
        client.readable().await?;

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
