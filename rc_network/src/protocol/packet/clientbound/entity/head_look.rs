use crate::protocol::data::read_types::{read_unsignedbyte, read_varint};
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct EntityHeadLookPacket {
    pub entity_id: i32,
    pub yaw: u8,
}

impl ClientBoundPacketType for EntityHeadLookPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_varint(buf);
        let yaw = read_unsignedbyte(buf);

        Box::new(EntityHeadLookPacket { entity_id, yaw })
    }
}
