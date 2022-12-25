
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PingResponse {
    pub ping: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ServerResponse {
    Ping(PingResponse),
}
