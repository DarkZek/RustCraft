use crate::error::ClientError;
use bevy::log::info;
use rc_protocol::stream::GameStream;
use std::net::{IpAddr, TcpStream};

pub struct ClientListener {
    pub stream: Option<GameStream>,
    pub disconnect: bool,
}

impl ClientListener {
    pub fn new() -> Result<ClientListener, ClientError> {
        Ok(ClientListener {
            stream: None,
            disconnect: false,
        })
    }

    pub fn stream(&self) -> Option<&TcpStream> {
        self.stream.as_ref().map(|v| &v.stream)
    }
}