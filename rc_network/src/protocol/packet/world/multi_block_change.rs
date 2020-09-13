use crate::protocol::data::read_types::{read_int, read_unsignedbyte, read_varint};
use crate::protocol::packet::PacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct MultiBlockChangePacket {
    pub x: i32,
    pub z: i32,
    pub changes: Vec<(u8, u8, u8, i64)>,
}

impl PacketType for MultiBlockChangePacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let x = read_int(buf);
        let z = read_int(buf);

        let changes_len = read_varint(buf);
        let mut changes = Vec::new();

        for _ in 0..changes_len {
            let pos = read_unsignedbyte(buf);
            let x = (pos >> 4 & 15);
            let z = (pos & 15);
            let y = read_unsignedbyte(buf);
            let block_id = read_varint(buf);

            changes.push((x, y, z, block_id));
        }

        Box::new(MultiBlockChangePacket { x, z, changes })
    }
}
