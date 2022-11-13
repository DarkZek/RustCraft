use bevy_log::debug;
use rc_protocol::types::SendPacket;
use crate::client::ClientSocket;

impl ClientSocket {
    pub fn send_packet(&mut self, packet: SendPacket) {
        debug!("<- {:?}", packet.0);
        // Lookup user
        self.write_packets.send(packet).unwrap();
    }
}