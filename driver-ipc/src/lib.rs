use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Command {
    // Single line of communication client->server
    // Driver commands
    //
    // Notify of monitor changes (whether adding or updating)
    DriverNotify(Vec<Monitor>),
    // Remove a monitor from system
    DriverRemove(Vec<Id>),
    // Remove all monitors from system
    DriverRemoveAll,
    // Requests
    // client->server
    //
    // Request information on the current system monitor state
    RequestState,
    // Replies to request
    // server->client
    ReplyState(Vec<Monitor>),
}
