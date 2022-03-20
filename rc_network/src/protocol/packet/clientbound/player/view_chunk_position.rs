use crate::protocol::data::read_types::read_varint;
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct UpdateViewChunkPositionPacket {
    pub x: i32,
    pub z: i32,
}

impl ClientBoundPacketType for UpdateViewChunkPositionPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let x = read_varint(buf);
        let z = read_varint(buf);

        Box::new(UpdateViewChunkPositionPacket { x, z })
    }
}
