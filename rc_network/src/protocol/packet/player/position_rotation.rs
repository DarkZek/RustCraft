use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_unsignedbyte, read_bool, read_int, read_float, read_double};
use std::io::{Cursor};

#[derive(Debug)]
pub struct PlayerPositionRotationPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

impl PacketType for PlayerPositionRotationPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let x = read_double(buf);
        let y = read_double(buf);
        let z = read_double(buf);
        let yaw = read_float(buf);
        let pitch = read_float(buf);
        let on_ground = read_bool(buf);

        Box::new(PlayerPositionRotationPacket {
            x,
            y,
            z,
            yaw,
            pitch,
            on_ground
        })
    }
}
