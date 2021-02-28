use crate::protocol::data::read_types::{read_bool, read_int, read_position};
use crate::protocol::packet::PacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct PlayEffectPacket {
    pub effect_id: i32,
    pub pos: [i64; 3],
    pub data: i32,
    pub disable_relative_volume: bool,
}

impl PacketType for PlayEffectPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let effect_id = read_int(buf);
        let pos = read_position(buf);
        let data = read_int(buf);
        let disable_relative_volume = read_bool(buf);

        Box::new(PlayEffectPacket {
            effect_id,
            pos,
            data,
            disable_relative_volume,
        })
    }
}
