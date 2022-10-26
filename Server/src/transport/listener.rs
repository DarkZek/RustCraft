use std::net::{IpAddr};
use bevy_log::info;
use mio::net::TcpListener;
use mio::{Interest, Poll};
use crate::ServerError;
use crate::transport::connection::SERVER;

pub struct ServerListener {
    pub stream: TcpListener,

    pub(crate) poll: Poll
}

impl ServerListener {
    pub fn new(ip: IpAddr, port: usize) -> Result<ServerListener, ServerError> {
        let mut stream = TcpListener::bind(format!("{}:{}", ip, port).parse().unwrap())?;

        let poll = Poll::new().unwrap();

        poll.registry()
            .register(&mut stream, SERVER, Interest::READABLE)?;

        info!("Listening for events on {}:{}", ip, port);

        Ok(ServerListener {
            stream,
            poll
        })
    }

    pub fn stream(&self) -> &TcpListener {
        &self.stream
    }
}