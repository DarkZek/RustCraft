use crate::protocol::data::read_types::{read_short, read_varint};
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct EntityVelocityPacket {
    pub entity_id: i32,
    pub velocity: [i16; 3],
}

impl ClientBoundPacketType for EntityVelocityPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_varint(buf);
        let velocity = [read_short(buf), read_short(buf), read_short(buf)];

        Box::new(EntityVelocityPacket {
            entity_id,
            velocity,
        })
    }
}
