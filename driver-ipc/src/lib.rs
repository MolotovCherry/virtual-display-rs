use serde::{Deserialize, Serialize};

pub type Id = u32;
pub type Dimen = u32;
pub type RefreshRate = u32;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Monitor {
    // identifier
    pub id: Id,
    pub enabled: bool,
    pub modes: Vec<Mode>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mode {
    pub width: Dimen,
    pub height: Dimen,
    pub refresh_rates: Vec<RefreshRate>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Command {
    // Single line of communication client->server
    // Driver commands
    DriverAdd(Vec<Monitor>),
    DriverRemove(Vec<Id>),
    DriverRemoveAll,
    // Requests
    // client->server
    RequestState,
    // Replies to request
    // server->client
    ReplyState(Vec<Monitor>),
}
