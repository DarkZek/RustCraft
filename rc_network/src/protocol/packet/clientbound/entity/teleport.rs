use crate::protocol::data::read_types::{read_bool, read_double, read_unsignedbyte, read_varint};
use crate::protocol::packet::clientbound::ClientBoundPacketType;

use std::io::Cursor;

// https://wiki.vg/Protocol#Entity_Position

#[derive(Debug)]
pub struct EntityTeleportPacket {
    pub entity_id: i32,
    pub pos: [f64; 3],
    pub yaw: u8,
    pub pitch: u8,
    pub on_ground: bool,
}

impl ClientBoundPacketType for EntityTeleportPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_varint(buf);
        let pos = [read_double(buf), read_double(buf), read_double(buf)];
        let yaw = read_unsignedbyte(buf);
        let pitch = read_unsignedbyte(buf);
        let on_ground = read_bool(buf);

        Box::new(EntityTeleportPacket {
            entity_id,
            pos,
            yaw,
            pitch,
            on_ground,
        })
    }
}