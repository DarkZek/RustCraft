use std::time::Duration;
use bevy_log::debug;
use rc_protocol::types::ReceivePacket;
use crate::client::ClientSocket;

pub struct ClientPollResult {
    pub packets: Vec<ReceivePacket>
}

impl ClientSocket {
    pub fn poll(&self) -> ClientPollResult {
        let packets = self.read_events();

        ClientPollResult {
            packets
        }
    }

    pub fn read_events(&self) -> Vec<ReceivePacket> {
        let mut packets = Vec::new();

        while let Ok(packet) = self.read_packets.recv_timeout(Duration::ZERO) {
            debug!("-> {:?}", packet.0);
            packets.push(packet);
        }

        packets
    }
}