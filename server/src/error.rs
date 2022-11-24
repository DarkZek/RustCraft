use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("data store disconnected")]
    Serde(#[from] serde_json::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
