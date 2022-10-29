use crate::error::ClientError;
use bevy::log::info;
use mio::net::{TcpStream};
use mio::{Token};
use rustcraft_protocol::stream::GameStream;
use std::net::IpAddr;

pub const CLIENT: Token = Token(0);

pub struct ClientListener {
    pub stream: Option<GameStream>,
    pub disconnect: bool,
}

impl ClientListener {
    pub fn new(ip: IpAddr, port: usize) -> Result<ClientListener, ClientError> {
        let stream = TcpStream::connect(format!("{}:{}", ip, port).parse().unwrap())?;

        info!("Connecting to server on {}:{}", ip, port);

        Ok(ClientListener {
            stream: Some(GameStream::new(stream)),
            disconnect: false,
        })
    }

    pub fn stream(&self) -> Option<&TcpStream> {
        self.stream.as_ref().map(|v| &v.stream)
    }
}
