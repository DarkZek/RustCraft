use crate::protocol::data::read_types::{read_bool, read_int, read_short, read_varint};
use crate::protocol::packet::PacketType;
use crate::protocol::types::entity_metadata::EntityMetadata;
use std::io::Cursor;

// https://wiki.vg/Protocol#Entity_Position

#[derive(Debug)]
pub struct UpdateEntityPositionPacket {
    pub entity_id: i64,
    pub change: [i16; 3],
    pub on_ground: bool,
}

impl PacketType for UpdateEntityPositionPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_varint(buf);
        let change = [read_short(buf), read_short(buf), read_short(buf)];
        let on_ground = read_bool(buf);

        Box::new(UpdateEntityPositionPacket {
            entity_id,
            change,
            on_ground,
        })
    }
}
