mod client;
mod core;
mod driver_client;
pub mod sync;
mod utils;

pub use client::Client;
pub use core::*;
pub use driver_client::DriverClient;

#[cfg(test)]
mod mock;

pub static DEFAULT_PIPE_NAME: &str = "virtualdisplaydriver";
