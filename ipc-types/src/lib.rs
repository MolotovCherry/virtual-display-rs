use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Monitor {
    // identifier
    pub id: u32,
    pub properties: Vec<MonitorProperties>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonitorProperties {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DriverCommand {
    Add(Vec<Monitor>),
    Remove(Vec<u32>),
    Update(Vec<Monitor>),
    RemoveAll,
}
