use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_float, read_varint};
use std::io::{Cursor};

#[derive(Debug)]
pub struct UpdatePlayerHealthPacket {
    pub health: f32,
    pub food: i64,
    pub food_saturation: f32
}

impl PacketType for UpdatePlayerHealthPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let health = read_float(buf);
        let food = read_varint(buf);
        let food_saturation = read_float(buf);

        Box::new(UpdatePlayerHealthPacket {
            health,
            food,
            food_saturation
        })
    }
}
