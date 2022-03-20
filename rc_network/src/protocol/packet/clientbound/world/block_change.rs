use crate::protocol::data::read_types::{read_position, read_varint};
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct BlockChangePacket {
    pub location: [i64; 3],
    pub block_id: i32,
}

impl ClientBoundPacketType for BlockChangePacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let location = read_position(buf);
        let block_id = read_varint(buf);

        Box::new(BlockChangePacket { location, block_id })
    }
}
