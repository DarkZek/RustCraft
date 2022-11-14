use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Disconnection")]
    Disconnected,
    #[error("Std IO error")]
    Io(#[from] io::Error),
    #[error("Std IO error")]
    Bincode(#[from] bincode::Error),
    #[error("Unknown error")]
    Unknown,
}