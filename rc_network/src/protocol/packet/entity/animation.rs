use crate::protocol::data::read_types::{read_unsignedbyte, read_varint};
use crate::protocol::packet::PacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct EntityAnimationPacket {
    pub entity_id: i64,
    pub animation: u8,
}

impl PacketType for EntityAnimationPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_varint(buf);
        let animation = read_unsignedbyte(buf);

        Box::new(EntityAnimationPacket {
            entity_id,
            animation,
        })
    }
}
