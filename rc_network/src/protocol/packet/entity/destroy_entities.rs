use crate::protocol::data::read_types::{read_varint};
use crate::protocol::packet::PacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct DestroyEntitiesPacket {
    pub entities: Vec<i64>,
}

impl PacketType for DestroyEntitiesPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let count = read_varint(buf);
        let mut entities = Vec::new();

        for _ in 0..count {
            entities.push(read_varint(buf));
        }

        Box::new(DestroyEntitiesPacket { entities })
    }
}
