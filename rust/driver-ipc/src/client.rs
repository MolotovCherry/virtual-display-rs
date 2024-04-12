use std::{
    convert::Infallible,
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread,
};

use log::error;
use serde::Serialize;
use tokio::{
    net::windows::named_pipe::{ClientOptions, NamedPipeClient, PipeMode},
    runtime::{Builder, Runtime},
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver},
        Mutex,
    },
};
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_WRITE},
    RegKey,
};

use crate::{
    utils::LazyLock, ClientCommand, DriverCommand, EventCommand, Id, IpcError, Monitor,
    ReplyCommand, RequestCommand, Result,
};

// EOF byte used to separate messages
const EOF: u8 = 0x4;

pub(crate) static RUNTIME: LazyLock<Runtime> =
    LazyLock::new(|| Builder::new_multi_thread().enable_all().build().unwrap());

/// A thin api client over the driver api with all the essential api.
/// Does not track state for you
///
/// This is cloneable and won't drop the client connection until all
/// instances are dropped
#[derive(Debug, Clone)]
pub struct Client {
    client: Arc<NamedPipeClient>,
    is_event: Arc<AtomicBool>,
    is_event_async: Arc<AtomicBool>,
    event_recv: Arc<Mutex<Receiver<EventCommand>>>,
    event_recv_async: Arc<Mutex<UnboundedReceiver<EventCommand>>>,
    client_recv: Arc<Mutex<Receiver<ReplyCommand>>>,
}

impl Client {
    /// connect to pipe virtualdisplaydriver
    pub fn connect() -> Result<Self> {
        Self::connect_to("virtualdisplaydriver")
    }

    // choose which pipe name you connect to
    // pipe name is ONLY the name, only the {name} portion of \\.\pipe\{name}
    pub fn connect_to(name: &str) -> Result<Self> {
        let fut = async {
            ClientOptions::new()
                .read(true)
                .write(true)
                .pipe_mode(PipeMode::Byte)
                .open(format!(r"\\.\pipe\{name}"))
                .map_err(IpcError::ConnectionFailed)
        };

        let client = Arc::new(RUNTIME.block_on(fut)?);
        let is_event = Arc::new(AtomicBool::new(false));
        let is_event_async = Arc::new(AtomicBool::new(false));
        let (event_send, event_recv) = channel();
        let (event_send_async, event_recv_async) = unbounded_channel();
        let (client_send, client_recv) = channel();

        let slf = Self {
            client: client.clone(),
            is_event: is_event.clone(),
            is_event_async: is_event_async.clone(),
            event_recv: Arc::new(Mutex::new(event_recv)),
            event_recv_async: Arc::new(Mutex::new(event_recv_async)),
            client_recv: Arc::new(Mutex::new(client_recv)),
        };

        let (tx, rx) = channel();

        // receive command thread
        thread::spawn(move || {
            _ = RUNTIME.block_on(receive_command(&client, tx));
        });

        thread::spawn(move || loop {
            let Ok(data) = rx.recv() else {
                // sender closed
                break;
            };

            let is_event = is_event.load(Ordering::Acquire);
            let is_event_async = is_event_async.load(Ordering::Acquire);
            let data_is_event = matches!(data, ClientCommand::Event(_));

            if (is_event || is_event_async) && data_is_event {
                let ClientCommand::Event(e) = data else {
                    unreachable!()
                };

                if is_event {
                    _ = event_send.send(e.clone());
                }

                if is_event_async {
                    _ = event_send_async.send(e);
                }
            } else if let ClientCommand::Reply(r) = data {
                _ = client_send.send(r);
            }
        });

        Ok(slf)
    }

    /// Notifies driver of changes (additions/updates/removals)
    pub fn notify(&self, monitors: &[Monitor]) -> Result<()> {
        let command = DriverCommand::Notify(monitors.to_owned());

        send_command(&self.client, &command)
    }

    /// Remove specific monitors by id
    pub fn remove(&self, ids: &[Id]) -> Result<()> {
        let command = DriverCommand::Remove(ids.to_owned());

        send_command(&self.client, &command)
    }

    /// Remove all monitors
    pub fn remove_all(&self) -> Result<()> {
        let command = DriverCommand::RemoveAll;

        send_command(&self.client, &command)
    }

    /// Receive generic reply
    ///
    /// If `last` is false, will only receive new messages from the point of calling
    /// If `last` is true, will receive the the last message received, or if none, blocks until the next one
    ///
    /// The reason for the `last` flag is that replies are auto buffered in the background, so if you send a
    /// request, the reply may be missed
    pub fn receive_reply(&self, last: bool) -> Option<ReplyCommand> {
        let lock = self.client_recv.blocking_lock();

        if last {
            last_recv(&lock)
        } else {
            latest_recv(&lock)
        }
    }

    /// Receive an event. Only new events after calling this are received
    pub fn receive_event(&self) -> EventCommand {
        self.is_event.store(true, Ordering::Release);

        let lock = self.event_recv.blocking_lock();
        let event = latest_recv(&lock);

        self.is_event.store(false, Ordering::Release);

        event.unwrap()
    }

    /// Receive an event. Only new events after calling this are received
    pub async fn receive_event_async(&self) -> EventCommand {
        self.is_event_async.store(true, Ordering::Release);

        let mut lock = self.event_recv_async.lock().await;
        let event = latest_event_recv_await(&mut lock).await;

        self.is_event_async.store(false, Ordering::Release);

        event.unwrap()
    }

    /// Request state update
    /// use `receive()` to get the reply
    pub fn request_state(&self) -> Result<()> {
        let command = RequestCommand::State;

        send_command(&self.client, &command)
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

fn send_command(client: &NamedPipeClient, command: &impl Serialize) -> Result<()> {
    // Create a vector with the full message, then send it as a single
    // write. This is required because the pipe is in message mode.
    let mut message = serde_json::to_vec(command)?;
    message.push(EOF);

    // write to pipe without needing to block or split it
    let fut = async {
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
                    return Err(e);
                }

                _ => unreachable!(),
            }
        }

        std::io::Result::Ok(())
    };

    RUNTIME.block_on(fut)?;

    Ok(())
}

// receive all commands and send them back to the receiver
async fn receive_command(
    client: &NamedPipeClient,
    tx: Sender<ClientCommand>,
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

/// Drains the channel and returns last message in the queue. If channel was empty, blocks for the next message
fn last_recv<T>(receiver: &Receiver<T>) -> Option<T> {
    use std::sync::mpsc::TryRecvError;

    let mut buf = None;

    loop {
        match receiver.try_recv() {
            Ok(t) => buf = Some(t),

            Err(TryRecvError::Empty) => {
                break if buf.is_none() {
                    receiver.recv().ok()
                } else {
                    buf
                };
            }

            Err(TryRecvError::Disconnected) => break None,
        }
    }
}

/// Drains the channel and blocks for the next message
fn latest_recv<T>(receiver: &Receiver<T>) -> Option<T> {
    use std::sync::mpsc::TryRecvError;

    loop {
        match receiver.try_recv() {
            Ok(_) => (),

            Err(TryRecvError::Empty) => break receiver.recv().ok(),

            Err(TryRecvError::Disconnected) => break None,
        }
    }
}

/// Drains the channel and blocks for the next message
async fn latest_event_recv_await<T>(receiver: &mut UnboundedReceiver<T>) -> Option<T> {
    use tokio::sync::mpsc::error::TryRecvError;

    loop {
        match receiver.try_recv() {
            Ok(_) => (),

            Err(TryRecvError::Empty) => break receiver.recv().await,

            Err(TryRecvError::Disconnected) => break None,
        }
    }
}
