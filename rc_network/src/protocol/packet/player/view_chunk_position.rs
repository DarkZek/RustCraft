use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_varint};
use std::io::{Cursor};

#[derive(Debug)]
pub struct UpdateViewChunkPositionPacket {
    pub x: i64,
    pub z: i64,
}

impl PacketType for UpdateViewChunkPositionPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let x = read_varint(buf);
        let z = read_varint(buf);

        Box::new(UpdateViewChunkPositionPacket {
            x,
            z,
        })
    }
}
