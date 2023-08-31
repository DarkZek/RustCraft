use crate::bistream::{BiStream, StreamError};
use quinn::Connection;
use tokio::sync::mpsc::UnboundedReceiver;

/// Stores a users connection details
pub struct UserConnection {
    pub connection: Connection,
    pub unreliable: BiStream,
    pub reliable: BiStream,
    pub chunk: BiStream,
    pub recv_err: UnboundedReceiver<StreamError>,
}
