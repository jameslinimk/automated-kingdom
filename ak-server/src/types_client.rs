use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ping {
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rename {
    pub name: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGame {
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientRequest {
    Ping(Ping),
    Rename(Rename),
    CreateGame(CreateGame),
}
impl ClientRequest {
    /// Returns true if the request should be rate limited
    pub fn ratelimited(&self) -> bool {
        !matches!(self, ClientRequest::Ping(_))
    }

    /// Returns the timestamp of the request
    pub fn timestamp(&self) -> u64 {
        macro_rules! timestamp {
            ($($x:ident),*) => {
                $(
                    if let ClientRequest::$x(x) = self {
                        return x.timestamp;
                    }
                )*
                unreachable!();
            };
        }

        timestamp!(Ping, Rename, CreateGame);
    }
}
