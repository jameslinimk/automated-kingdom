use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ErrorCode {
    /// The username contains invalid characters
    UsernameInvalid,
    /// The username is too long
    UsernameTooLong,
    /// Sent request too fast
    Ratelimited,
    AlreadyInGame,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ServerResponseData {
    Error(ErrorCode),
    GameCreateSuccess(Uuid),
    Success,
}

/// `(response, ping)`
pub type ServerResponse = (ServerResponseData, u16);
