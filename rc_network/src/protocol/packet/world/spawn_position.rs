use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_position};
use std::io::{Cursor, Seek, Read};

#[derive(Debug)]
pub struct SpawnPositionPacket {
    pub pos: [i64; 3],
}

impl PacketType for SpawnPositionPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let pos = read_position(buf);

        Box::new(SpawnPositionPacket {
            pos,
        })
    }
}
