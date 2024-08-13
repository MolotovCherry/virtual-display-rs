use flutter_rust_bridge::frb;

mod ipc {
    pub use driver_ipc::mock::*;
    pub use driver_ipc::*;
}

#[frb(opaque)]
pub struct MockServer {
    server: ipc::MockServer,
    pipe_name: String,
}

impl MockServer {
    pub async fn create(pipe_name: String) -> Self {
        Self {
            server: ipc::MockServer::new(&pipe_name).await,
            pipe_name,
        }
    }

    #[frb(getter, sync)]
    pub fn pipe_name(&self) -> String {
        self.pipe_name.clone()
    }

    #[frb(getter, sync)]
    pub fn state(&self) -> Vec<ipc::Monitor> {
        self.server.state().to_owned()
    }

    pub async fn set_state(&mut self, state: Vec<ipc::Monitor>) {
        self.server.set_state(state).await;
    }

    pub async fn pump(&mut self) {
        self.server.pump().await;
    }
}
