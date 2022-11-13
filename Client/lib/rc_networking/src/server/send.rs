use rc_protocol::types::SendPacket;
use crate::server::ServerSocket;

impl ServerSocket {
    pub fn send_packet(&mut self, packet: SendPacket) {
        // Lookup user
        if let Some(user) = self.users.get(&packet.1) {
            user.write_packets.send(packet);
        }
    }
}