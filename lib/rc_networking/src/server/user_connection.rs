use crate::bistream::{BiStream, StreamError};
use tokio::sync::mpsc::UnboundedReceiver;
use web_transport::Session;

/// Stores a users connection details
pub struct UserConnection {
    pub connection: Session,
    pub unreliable: BiStream,
    pub reliable: BiStream,
    pub chunk: BiStream,
    pub recv_err: UnboundedReceiver<StreamError>,
    pub user_id: u64
}
