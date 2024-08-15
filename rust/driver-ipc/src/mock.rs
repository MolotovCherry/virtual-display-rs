use std::{io, mem, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::windows::named_pipe,
    sync::{broadcast, Notify},
    task,
};

use crate::*;

use self::client::EOF;

pub struct MockServer {
    server: Arc<named_pipe::NamedPipeServer>,
    state: Vec<Monitor>,
    command_rx: broadcast::Receiver<ServerCommand>,
    command_tx: broadcast::Sender<ServerCommand>,
    notify_closed: Arc<Notify>,
}

impl MockServer {
    /// async because needs a tokio reactor to run. Will return immediately.
    pub async fn new(name: &str) -> Self {
        let server = named_pipe::ServerOptions::new()
            .access_inbound(true)
            .access_outbound(true)
            .reject_remote_clients(true)
            .create(format!(r"\\.\pipe\{}", name))
            .unwrap();
        let server = Arc::new(server);

        let notify_closed = Arc::new(Notify::new());

        let (command_tx, command_rx) = broadcast::channel(64);

        {
            let server = server.clone();
            let command_tx = command_tx.clone();
            let notify_closed = notify_closed.clone();
            task::spawn(async move {
                let server = unsafe {
                    (server.as_ref() as *const _ as *mut named_pipe::NamedPipeServer)
                        .as_mut()
                        .unwrap()
                };

                server.connect().await.expect("Failed to connect to server");

                loop {
                    let mut buf = vec![];
                    loop {
                        let byte = tokio::select! {
                            _ = notify_closed.notified() => return,
                            r = server.read_u8() => r,
                        };

                        let v = match byte {
                            Ok(v) => v,
                            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return, // Client disconnected
                            Err(_) => continue,
                        };
                        if v == EOF {
                            break;
                        }
                        buf.push(v);
                    }

                    let cmd = serde_json::from_slice::<ServerCommand>(&buf)
                        .expect("Failed to deserialize request");

                    command_tx.send(cmd).expect("Failed to send command");
                }
            });
        }

        Self {
            server,
            state: vec![],
            command_rx,
            command_tx,
            notify_closed,
        }
    }

    pub fn state(&self) -> &[Monitor] {
        &self.state
    }

    pub async fn set_state(&mut self, state: Vec<Monitor>) {
        let state = mem::replace(&mut self.state, state);
        if state != self.state {
            self.send_changed_event().await;
        }
    }

    pub fn check_next(&mut self, cb: impl FnOnce(ServerCommand) + Send + 'static) {
        let mut rx = self.command_tx.subscribe();

        task::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(cmd) => {
                        cb(cmd);
                        break;
                    }
                    Err(_) => continue,
                }
            }
        });
    }

    pub async fn pump(&mut self) {
        let cmd = self.command_rx.recv().await.unwrap();

        let server = unsafe {
            (self.server.as_ref() as *const _ as *mut named_pipe::NamedPipeServer)
                .as_mut()
                .unwrap()
        };

        let changed = match cmd {
            ServerCommand::Request(RequestCommand::State) => {
                let reply = ReplyCommand::State(self.state.clone());
                let mut reply = serde_json::to_vec(&reply).unwrap();
                reply.push(EOF);

                server
                    .write_all(&reply)
                    .await
                    .expect("Failed to write reply");
                false
            }
            ServerCommand::Driver(DriverCommand::Notify(monitors)) => {
                self.state = monitors;
                true
            }
            ServerCommand::Driver(DriverCommand::Remove(ids)) => {
                self.state.retain(|m| !ids.contains(&m.id));
                true
            }
            ServerCommand::Driver(DriverCommand::RemoveAll) => {
                self.state.clear();
                true
            }
        };

        if changed {
            self.send_changed_event().await;
        }
    }

    async fn send_changed_event(&self) {
        let server = unsafe {
            (self.server.as_ref() as *const _ as *mut named_pipe::NamedPipeServer)
                .as_mut()
                .unwrap()
        };

        let event = EventCommand::Changed(self.state.clone());
        let mut event = serde_json::to_vec(&event).unwrap();
        event.push(EOF);

        server
            .write_all(&event)
            .await
            .expect("Failed to write event");
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.notify_closed.notify_waiters();
    }
}
