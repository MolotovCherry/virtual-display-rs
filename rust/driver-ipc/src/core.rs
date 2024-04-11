use serde::{Deserialize, Serialize};

pub type Id = u32;
pub type Dimen = u32;
pub type RefreshRate = u32;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Monitor {
    // identifier
    pub id: Id,
    pub name: Option<String>,
    pub enabled: bool,
    pub modes: Vec<Mode>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
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
    // Reply to previous current system monitor state request
    State(Vec<Monitor>),
}

/// An event happened
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum EventCommand {
    // Monitor state was changed while client was connected
    Changed(Vec<Monitor>),
}

/// An untagged enum of commands to be used with deserialization.
/// This makes the deserialization process much easier to handle
/// when a received command could be of multiple types
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCommand {
    Driver(DriverCommand),
    Request(RequestCommand),
}

/// An untagged enum of commands to be used with deserialization.
/// This makes the deserialization process much easier to handle
/// when a received command could be of multiple types
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientCommand {
    Reply(ReplyCommand),
    Event(EventCommand),
}
