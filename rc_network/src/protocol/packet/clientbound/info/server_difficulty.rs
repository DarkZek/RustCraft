use crate::protocol::data::read_types::{read_bool, read_unsignedbyte};
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct ServerDifficultyPacket {
    pub difficulty: u8,
    pub difficulty_locked: bool,
}

impl ClientBoundPacketType for ServerDifficultyPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let difficulty = read_unsignedbyte(buf);
        let difficulty_locked = read_bool(buf);

        Box::new(ServerDifficultyPacket {
            difficulty,
            difficulty_locked,
        })
    }
}
