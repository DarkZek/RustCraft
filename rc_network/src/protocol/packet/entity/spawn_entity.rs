use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_unsignedbyte, read_bool, read_int, read_short, read_double, read_varint, read_uuid};
use std::io::{Cursor, Seek, Read};

#[derive(Debug)]
pub struct SpawnEntityPacket {
    pub entity_id: i64,
    pub entity_uuid: u128,
    pub ty: i64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: u8,
    pub yaw: u8,
    pub data: i32,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16
}

impl PacketType for SpawnEntityPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_varint(buf);
        let entity_uuid = read_uuid(buf);
        let ty = read_varint(buf);
        let x = read_double(buf);
        let y = read_double(buf);
        let z = read_double(buf);
        let pitch = read_unsignedbyte(buf);
        let yaw = read_unsignedbyte(buf);
        let data = read_int(buf);
        let velocity_x = read_short(buf);
        let velocity_y = read_short(buf);
        let velocity_z = read_short(buf);

        Box::new(SpawnEntityPacket {
            entity_id,
            entity_uuid,
            ty,
            x,
            y,
            z,
            yaw,
            pitch,
            data,
            velocity_x,
            velocity_y,
            velocity_z
        })
    }
}
