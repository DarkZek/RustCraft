use crate::protocol::data::read_types::{read_unsignedbyte, read_varint};
use crate::protocol::packet::PacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct UpdateLightLevelsPacket {
    pub x: i32,
    pub z: i32,
    pub sky_light_mask: i32,
    pub block_light_mask: i32,
    pub empty_sky_light_mask: i32,
    pub empty_block_light_mask: i32,
    pub sky_light: Option<Vec<u8>>,
    pub block_light: Option<Vec<u8>>,
}

impl PacketType for UpdateLightLevelsPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let x = read_varint(buf);
        let z = read_varint(buf);
        let sky_light_mask = read_varint(buf);
        let block_light_mask = read_varint(buf);
        let empty_sky_light_mask = read_varint(buf);
        let empty_block_light_mask = read_varint(buf);

        // Check this later
        if buf.get_ref().len() - buf.position() as usize == 0 {
            return Box::new(UpdateLightLevelsPacket {
                x,
                z,
                sky_light_mask,
                block_light_mask,
                empty_sky_light_mask,
                empty_block_light_mask,
                sky_light: None,
                block_light: None,
            });
        }

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
            sky_light: Some(sky_light),
            block_light: Some(block_light),
        })
    }
}
