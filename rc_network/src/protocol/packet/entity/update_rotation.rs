use crate::protocol::data::read_types::{
    read_bool, read_unsignedbyte, read_varint,
};
use crate::protocol::packet::PacketType;

use std::io::Cursor;

// https://wiki.vg/Protocol#Entity_Position

#[derive(Debug)]
pub struct UpdateEntityRotationPacket {
    pub entity_id: i64,
    pub change_yaw: u8,
    pub change_pitch: u8,
    pub on_ground: bool,
}

impl PacketType for UpdateEntityRotationPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_varint(buf);
        let change_yaw = read_unsignedbyte(buf);
        let change_pitch = read_unsignedbyte(buf);
        let on_ground = read_bool(buf);

        Box::new(UpdateEntityRotationPacket {
            entity_id,
            change_yaw,
            change_pitch,
            on_ground,
        })
    }
}
