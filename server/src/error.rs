use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("data store disconnected")]
    Serde(#[from] serde_json::Error),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
