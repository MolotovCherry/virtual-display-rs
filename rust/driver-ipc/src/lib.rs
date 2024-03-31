mod client;

use serde::{Deserialize, Serialize};

pub use client::Client;

pub type Id = u32;
pub type Dimen = u32;
pub type RefreshRate = u32;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Monitor {
    // identifier
    pub id: Id,
    pub name: Option<String>,
    pub enabled: bool,
    pub modes: Vec<Mode>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Mode {
    pub width: Dimen,
    pub height: Dimen,
    pub refresh_rates: Vec<RefreshRate>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DriverCommand {
    // Single line of communication client->server
    // Driver commands
    //
    // Notify of monitor changes (whether adding or updating)
    Notify(Vec<Monitor>),
    // Remove a monitor from system
    Remove(Vec<Id>),
    // Remove all monitors from system
    RemoveAll,
}

/// Request command sent from client->server
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RequestCommand {
    // Request information on the current system monitor state
    State,
}

/// Reply command sent from server->client
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ReplyCommand {
    // server->client
    State(Vec<Monitor>),
}

/// An untagged enum of commands to be used with deserialization.
/// This makes the deserialization process much easier to handle
/// when a received command could be of multiple types
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Command {
    Driver(DriverCommand),
    Request(RequestCommand),
    Reply(ReplyCommand),
}
