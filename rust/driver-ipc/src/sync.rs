mod client;
mod driver_client;

pub use client::Client;
pub use driver_client::DriverClient;

use tokio::runtime::{Builder, Runtime};

use crate::utils::LazyLock;

static RUNTIME: LazyLock<Runtime> =
    LazyLock::new(|| Builder::new_multi_thread().enable_all().build().unwrap());
