use crate::bistream::{BiStream, StreamError};
use tokio::sync::mpsc::UnboundedReceiver;
use web_transport::Session;
use crate::server::authorization::AuthorizationResult;

/// Stores a users connection details
pub struct UserConnection {
    pub connection: Session,
    pub unreliable: BiStream,
    pub reliable: BiStream,
    pub chunk: BiStream,
    pub recv_err: UnboundedReceiver<StreamError>,
    pub user_authorization: AuthorizationResult
}
