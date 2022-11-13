use std::time::Duration;
use bevy_log::{debug, error, info, warn};
use crossbeam::channel::{Receiver, Sender, unbounded};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use rc_protocol::constants::{EntityId, UserId};
use rc_protocol::protocol::clientbound::ping::Ping;
use rc_protocol::protocol::Protocol;
use rc_protocol::protocol::serverbound::pong::Pong;
use rc_protocol::types::{ReceivePacket, SendPacket};
use crate::server::connection::ConnectionEvent;
use crate::server::ServerSocket;
use crate::server::user::NetworkUser;

pub struct ConnectionRequest(pub TcpStream);

pub struct PollResult {
    pub connections: Vec<ConnectionEvent>,
    pub packets: Vec<ReceivePacket>
}

impl ServerSocket {
    pub fn poll(&mut self) -> PollResult {
        let connections = self.new_connections();

        let packets = self.read_events();

        PollResult {
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