use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Ping {
    pub time: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ClientRequest {
    Ping(Ping),
}
