use std::net::IpAddr;
use bevy::log::info;
use mio::net::{TcpListener, TcpStream};
use mio::{Interest, Poll, Token};
use rustcraft_protocol::stream::GameStream;
use crate::error::ClientError;

pub const CLIENT: Token = Token(0);

pub struct ClientListener {
    pub stream: Option<GameStream>,
}

impl ClientListener {
    pub fn new(ip: IpAddr, port: usize) -> Result<ClientListener, ClientError> {
        let mut stream = TcpStream::connect(format!("{}:{}", ip, port).parse().unwrap())?;

        info!("Connecting to server on {}:{}", ip, port);

        Ok(ClientListener {
            stream: Some(GameStream::new(stream))
        })
    }

    pub fn stream(&self) -> Option<&TcpStream> {
        self.stream.as_ref().map(|v| &v.stream)
    }
}
