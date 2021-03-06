use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_unsignedbyte, read_float, read_double, read_varint};
use std::io::{Cursor};

#[derive(Debug)]
pub struct PlayerPositionLookPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
    pub teleport_id: i64
}

impl PacketType for PlayerPositionLookPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let x = read_double(buf);
        let y = read_double(buf);
        let z = read_double(buf);
        let yaw = read_float(buf);
        let pitch = read_float(buf);
        let flags = read_unsignedbyte(buf);
        let teleport_id = read_varint(buf);

        Box::new(PlayerPositionLookPacket {
            x,
            y,
            z,
            yaw,
            pitch,
            flags,
            teleport_id
        })
    }
}
