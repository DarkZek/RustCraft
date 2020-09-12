use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_unsignedbyte, read_bool, read_int, read_float, read_double, read_varint};
use std::io::{Cursor, Seek, Read};

#[derive(Debug)]
pub struct UpdateLightLevelsPacket {
    pub x: i64,
    pub z: i64,
    pub sky_light_mask: i64,
    pub block_light_mask: i64,
    pub empty_sky_light_mask: i64,
    pub empty_block_light_mask: i64,
    pub sky_light: Vec<u8>,
    pub block_light: Vec<u8>,
}

impl PacketType for UpdateLightLevelsPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let x = read_varint(buf);
        let z = read_varint(buf);
        let sky_light_mask = read_varint(buf);
        let block_light_mask = read_varint(buf);
        let empty_sky_light_mask = read_varint(buf);
        let empty_block_light_mask = read_varint(buf);

        let sky_light_len = read_varint(buf);
        let mut sky_light = Vec::new();

        for _ in 0..sky_light_len {
            sky_light.push(read_unsignedbyte(buf));
        }

        let block_light_len = read_varint(buf);
        let mut block_light = Vec::new();

        for _ in 0..block_light_len {
            block_light.push(read_unsignedbyte(buf));
        }


        Box::new(UpdateLightLevelsPacket {
            x,
            z,
            sky_light_mask,
            block_light_mask,
            empty_sky_light_mask,
            empty_block_light_mask,
            sky_light,
            block_light
        })
    }
}
