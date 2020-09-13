use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_unsignedbyte, read_varint};
use std::io::{Cursor};

#[derive(Debug)]
pub struct EntityHeadLookPacket {
    pub entity_id: i64,
    pub yaw: u8
}

impl PacketType for EntityHeadLookPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_varint(buf);
        let yaw = read_unsignedbyte(buf);

        Box::new(EntityHeadLookPacket {
            entity_id,
            yaw
        })
    }
}
