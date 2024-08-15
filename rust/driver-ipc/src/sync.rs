pub mod client;
pub mod driver_client;

use std::sync::LazyLock;

pub use client::{Client, EventsSubscription};
pub use driver_client::DriverClient;

use tokio::runtime::{Builder, Runtime};

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap()
});
