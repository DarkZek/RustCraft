use crate::protocol::data::read_types::read_position;
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct SpawnPositionPacket {
    pub pos: [i64; 3],
}

impl ClientBoundPacketType for SpawnPositionPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let pos = read_position(buf);

        Box::new(SpawnPositionPacket { pos })
    }
}
