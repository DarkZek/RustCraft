use crate::protocol::data::read_types::{read_double, read_unsignedbyte, read_uuid, read_varint};
use crate::protocol::packet::PacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct SpawnPlayerPacket {
    pub entity_id: i32,
    pub player_uuid: u128,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: u8,
    pub pitch: u8,
}

impl PacketType for SpawnPlayerPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_varint(buf);
        let player_uuid = read_uuid(buf);
        let x = read_double(buf);
        let y = read_double(buf);
        let z = read_double(buf);
        let yaw = read_unsignedbyte(buf);
        let pitch = read_unsignedbyte(buf);

        Box::new(SpawnPlayerPacket {
            entity_id,
            player_uuid,
            x,
            y,
            z,
            yaw,
            pitch,
        })
    }
}
