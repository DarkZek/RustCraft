use std::time::Duration;
use bevy::log::{debug};

use tokio::net::TcpStream;

use rc_protocol::types::{ReceivePacket};
use crate::server::connection::ConnectionEvent;
use crate::server::ServerSocket;


pub struct ConnectionRequest(pub TcpStream);

pub struct ServerPollResult {
    pub connections: Vec<ConnectionEvent>,
    pub packets: Vec<ReceivePacket>
}

impl ServerSocket {
    pub fn poll(&mut self) -> ServerPollResult {
        let connections = self.new_connections();

        let packets = self.read_events();

        ServerPollResult {
            connections,
            packets
        }
    }

    pub fn read_events(&mut self) -> Vec<ReceivePacket> {
        let mut packets = Vec::new();

        for (_, user) in &self.users {
            while let Ok(packet) = user.read_packets.recv_timeout(Duration::ZERO) {
                debug!("-> {:?}", packet.0);
                packets.push(packet);
            }
        }

        packets
    }
}