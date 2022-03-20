use crate::protocol::data::read_types::read_float;
use crate::protocol::data::write_types::{write_bool, write_float};
use crate::protocol::packet::serverbound::{ServerBoundPacketData, ServerBoundPacketType};
use crate::PacketBuilder;
use std::io::Cursor;

#[derive(Debug)]
pub struct PlayerRotationPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

impl ServerBoundPacketType for PlayerRotationPacket {
    fn serialize(&self) -> PacketBuilder {
        let mut packet = PacketBuilder::new(0x13);

        write_float(self.yaw, &mut packet.data);
        write_float(self.pitch, &mut packet.data);
        write_bool(self.on_ground, &mut packet.data);

        packet
    }
}
