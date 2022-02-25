use crate::protocol::data::read_types::{
    read_double, read_short, read_unsignedbyte, read_uuid, read_varint,
};
use crate::protocol::packet::PacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct SpawnLivingEntityPacket {
    pub entity_id: i32,
    pub entity_uuid: u128,
    pub ty: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: u8,
    pub pitch: u8,
    pub head_pitch: u8,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}

impl PacketType for SpawnLivingEntityPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_varint(buf);
        let entity_uuid = read_uuid(buf);
        let ty = read_varint(buf);
        let x = read_double(buf);
        let y = read_double(buf);
        let z = read_double(buf);
        let yaw = read_unsignedbyte(buf);
        let pitch = read_unsignedbyte(buf);
        let head_pitch = read_unsignedbyte(buf);
        let velocity_x = read_short(buf);
        let velocity_y = read_short(buf);
        let velocity_z = read_short(buf);

        Box::new(SpawnLivingEntityPacket {
            entity_id,
            entity_uuid,
            ty,
            x,
            y,
            z,
            yaw,
            pitch,
            head_pitch,
            velocity_x,
            velocity_y,
            velocity_z,
        })
    }
}
