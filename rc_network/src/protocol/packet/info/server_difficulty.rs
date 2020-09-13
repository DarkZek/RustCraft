use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_unsignedbyte, read_bool};
use std::io::{Cursor};

#[derive(Debug)]
pub struct ServerDifficultyPacket {
    pub difficulty: u8,
    pub difficulty_locked: bool
}

impl PacketType for ServerDifficultyPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let difficulty = read_unsignedbyte(buf);
        let difficulty_locked = read_bool(buf);

        Box::new(ServerDifficultyPacket {
            difficulty,
            difficulty_locked
        })
    }
}
