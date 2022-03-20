use crate::protocol::data::read_types::{read_int, read_unsignedbyte};
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct EntityStatusPacket {
    pub entity_id: i32,
    pub entity_status: u8,
}

impl ClientBoundPacketType for EntityStatusPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_int(buf);
        let entity_status = read_unsignedbyte(buf);

        Box::new(EntityStatusPacket {
            entity_id,
            entity_status,
        })
    }
}
