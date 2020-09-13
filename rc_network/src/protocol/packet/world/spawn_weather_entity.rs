use crate::protocol::data::read_types::{read_double, read_int, read_unsignedbyte, read_varint};
use crate::protocol::packet::PacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct SpawnWeatherEntityPacket {
    pub entity_id: i64,
    pub ty: u8,
    pub pos: [f64; 3],
}

impl PacketType for SpawnWeatherEntityPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_varint(buf);
        let ty = read_unsignedbyte(buf);
        let pos = [read_double(buf), read_double(buf), read_double(buf)];

        Box::new(SpawnWeatherEntityPacket { entity_id, ty, pos })
    }
}
