use std::{
    convert::Infallible,
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use log::error;
use serde::Serialize;
use tokio::{
    net::windows::named_pipe::{ClientOptions, NamedPipeClient, PipeMode},
    sync::{
        mpsc::{error::TryRecvError, unbounded_channel, UnboundedReceiver, UnboundedSender},
        Mutex,
    },
    task,
};
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_WRITE},
    RegKey,
};

use crate::{
    ClientCommand, DriverCommand, EventCommand, Id, IpcError, Monitor, ReplyCommand,
    RequestCommand, Result,
};

// EOF byte used to separate messages
const EOF: u8 = 0x4;

/// A thin api client over the driver api with all the essential api.
/// Does the bare minimum required. Does not track state
#[derive(Debug, Clone)]
pub struct Client(Arc<Inner>);

#[derive(Debug)]
struct Inner {
    client: NamedPipeClient,
    user_is_event: AtomicBool,
    event_recv: Mutex<UnboundedReceiver<EventCommand>>,
    client_recv: Mutex<UnboundedReceiver<ReplyCommand>>,
}

impl Client {
    /// connect to pipe virtualdisplaydriver
    pub async fn connect() -> Result<Self> {
        Self::connect_to("virtualdisplaydriver").await
    }

    // choose which pipe name you connect to
    // pipe name is ONLY the name, only the {name} portion of \\.\pipe\{name}
    pub async fn connect_to(name: &str) -> Result<Self> {
        let client = ClientOptions::new()
            .read(true)
            .write(true)
            .pipe_mode(PipeMode::Byte)
            .open(format!(r"\\.\pipe\{name}"))
            .map_err(IpcError::ConnectionFailed)?;

        let client = client;
        let user_is_event = AtomicBool::new(false);
        let (event_send, event_recv) = unbounded_channel();
        let (client_send, client_recv) = unbounded_channel();

        let inner = Inner {
            client,
            user_is_event,
            event_recv: Mutex::new(event_recv),
            client_recv: Mutex::new(client_recv),
        };

        let client = Self(Arc::new(inner));

        let (tx, mut rx) = unbounded_channel();

        // receive command thread
        let client2 = client.clone();
        task::spawn(async move {
            _ = receive_command(&client2, tx).await;
        });

        let client2 = client.clone();
        task::spawn(async move {
            let user_is_event = &client2.0.user_is_event;

            loop {
                let Some(data) = rx.recv().await else {
                    // sender closed
                    break;
                };

                let user_is_event = user_is_event.load(Ordering::Acquire);
                let data_is_event = matches!(data, ClientCommand::Event(_));

                if user_is_event && data_is_event {
                    let ClientCommand::Event(e) = data else {
                        unreachable!()
                    };

                    if user_is_event {
                        _ = event_send.send(e.clone());
                    }
                } else if let ClientCommand::Reply(r) = data {
                    _ = client_send.send(r);
                }
            }
        });

        Ok(client)
    }

    /// Notifies driver of changes (additions/updates/removals)
    pub async fn notify(&self, monitors: &[Monitor]) -> Result<()> {
        let command = DriverCommand::Notify(monitors.to_owned());

        send_command(&self.0.client, &command).await
    }

    /// Remove specific monitors by id
    pub async fn remove(&self, ids: &[Id]) -> Result<()> {
        let command = DriverCommand::Remove(ids.to_owned());

        send_command(&self.0.client, &command).await
    }

    /// Remove all monitors
    pub async fn remove_all(&self) -> Result<()> {
        let command = DriverCommand::RemoveAll;

        send_command(&self.0.client, &command).await
    }

    /// Receive generic reply
    ///
    /// If `last` is false, will only receive new messages from the point of calling
    /// If `last` is true, will receive the the last message received, or if none, blocks until the next one
    ///
    /// The reason for the `last` flag is that replies are auto buffered in the background, so if you send a
    /// request, the reply may be missed
    pub async fn receive_reply(&self, last: bool) -> Option<ReplyCommand> {
        let mut recv = self.0.client_recv.lock().await;

        if last {
            last_recv(&mut recv).await
        } else {
            latest_recv(&mut recv).await
        }
    }

    /// Receive an event. Only new events after calling this are received
    pub async fn receive_event(&self) -> EventCommand {
        self.0.user_is_event.store(true, Ordering::Release);

        let mut recv = self.0.event_recv.lock().await;
        let event = latest_recv(&mut recv).await;

        self.0.user_is_event.store(false, Ordering::Release);

        event.unwrap()
    }

    /// Request state update
    /// use `receive()` to get the reply
    pub async fn request_state(&self) -> Result<()> {
        let command = RequestCommand::State;

        send_command(&self.0.client, &command).await
    }

    /// Persist changes to registry for current user
    pub fn persist(monitors: &[Monitor]) -> Result<()> {
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        let key = r"SOFTWARE\VirtualDisplayDriver";

        let mut reg_key = hklm.open_subkey_with_flags(key, KEY_WRITE);

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

async fn send_command(client: &NamedPipeClient, command: &impl Serialize) -> Result<()> {
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
    client: &Client,
    tx: UnboundedSender<ClientCommand>,
) -> Result<Infallible> {
    let mut buf = vec![0; 4096];
    let mut recv_buf = Vec::with_capacity(4096);

    let client = &client.0.client;

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

/// Drains the channel and returns last message in the queue. If channel was empty, blocks for the next message
async fn last_recv<T>(receiver: &mut UnboundedReceiver<T>) -> Option<T> {
    let mut buf = None;

    loop {
        match receiver.try_recv() {
            Ok(t) => buf = Some(t),

            Err(TryRecvError::Empty) => {
                break if buf.is_none() {
                    receiver.recv().await
                } else {
                    buf
                };
            }

            Err(TryRecvError::Disconnected) => break None,
        }
    }
}

/// Drains the channel and blocks for the next message
async fn latest_recv<T>(receiver: &mut UnboundedReceiver<T>) -> Option<T> {
    loop {
        match receiver.try_recv() {
            Ok(_) => (),

            Err(TryRecvError::Empty) => break receiver.recv().await,

            Err(TryRecvError::Disconnected) => break None,
        }
    }
}
