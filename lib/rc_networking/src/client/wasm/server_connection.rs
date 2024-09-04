use tokio::sync::mpsc::UnboundedReceiver;
use web_transport::wasm::Session;
use crate::bistream::{BiStream, StreamError};

pub struct ServerConnection {
    pub connection: Session,
    pub unreliable: BiStream,
    pub reliable: BiStream,
    pub chunk: BiStream,
    pub err_recv: UnboundedReceiver<StreamError>,
}
