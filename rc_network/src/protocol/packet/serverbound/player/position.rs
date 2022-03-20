use crate::protocol::data::read_types::read_float;
use crate::protocol::data::write_types::{write_bool, write_double, write_float};
use crate::protocol::packet::serverbound::{ServerBoundPacketData, ServerBoundPacketType};
use crate::PacketBuilder;
use std::io::Cursor;

#[derive(Debug)]
pub struct PlayerPositionPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool,
}

impl ServerBoundPacketType for PlayerPositionPacket {
    fn serialize(&self) -> PacketBuilder {
        let mut packet = PacketBuilder::new(0x11);

        write_double(self.x, &mut packet.data);
        write_double(self.y, &mut packet.data);
        write_double(self.z, &mut packet.data);
        write_bool(self.on_ground, &mut packet.data);

        packet
    }
}
