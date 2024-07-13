use crate::bistream::{BiStream, StreamError};
use quinn::Connection;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct ServerConnection {
    pub connection: Connection,
    pub unreliable: BiStream,
    pub reliable: BiStream,
    pub chunk: BiStream,
    pub err_recv: UnboundedReceiver<StreamError>,
}
