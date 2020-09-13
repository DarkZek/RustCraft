use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_unsignedbyte, read_float};
use std::io::{Cursor};

#[derive(Debug)]
pub struct PlayerAbilitiesPacket {
    pub flags: u8,
    pub flying_speed: f32,
    pub field_of_view_modifier: f32
}

impl PacketType for PlayerAbilitiesPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let flags = read_unsignedbyte(buf);
        let flying_speed = read_float(buf);
        let field_of_view_modifier = read_float(buf);

        Box::new(PlayerAbilitiesPacket {
            flags,
            flying_speed,
            field_of_view_modifier
        })
    }
}
