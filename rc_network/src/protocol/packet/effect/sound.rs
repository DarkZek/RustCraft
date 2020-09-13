use crate::protocol::data::read_types::{read_float, read_int, read_unsignedbyte, read_varint};
use crate::protocol::packet::PacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct SoundEffectPacket {
    pub sound_id: i64,
    pub sound_category: i64,
    pub pos: [i32; 3],
    pub volume: f32,
    pub pitch: f32,
}

impl PacketType for SoundEffectPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let sound_id = read_varint(buf);
        let sound_category = read_varint(buf);
        let pos = [read_int(buf), read_int(buf), read_int(buf)];
        let volume = read_float(buf);
        let pitch = read_float(buf);

        Box::new(SoundEffectPacket {
            sound_id,
            sound_category,
            pos,
            volume,
            pitch,
        })
    }
}
