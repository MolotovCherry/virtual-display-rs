pub mod client;
mod core;
pub mod driver_client;
pub mod sync;

pub use client::Client;
pub use core::*;
pub use driver_client::DriverClient;

#[cfg(test)]
mod mock;

pub static DEFAULT_PIPE_NAME: &str = "virtualdisplaydriver";
