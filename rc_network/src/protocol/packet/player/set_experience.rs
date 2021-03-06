use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_float, read_varint};
use std::io::{Cursor};

#[derive(Debug)]
pub struct SetPlayerExperiencePacket {
    pub experience_bar: f32,
    pub level: i64,
    pub total_experience: i64
}

impl PacketType for SetPlayerExperiencePacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let experience_bar = read_float(buf);
        let level = read_varint(buf);
        let total_experience = read_varint(buf);

        Box::new(SetPlayerExperiencePacket {
            experience_bar,
            level,
            total_experience
        })
    }
}
