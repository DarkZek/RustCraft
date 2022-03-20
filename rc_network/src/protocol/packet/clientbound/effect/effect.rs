use crate::protocol::data::read_types::{read_unsignedbyte, read_varint};
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct EntityEffectPacket {
    pub entity_id: i32,
    pub effect_id: u8,
    pub amplifier: u8,
    pub duration: i32,
    pub flags: u8,
}

impl ClientBoundPacketType for EntityEffectPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_varint(buf);
        let effect_id = read_unsignedbyte(buf);
        let amplifier = read_unsignedbyte(buf);
        let duration = read_varint(buf);
        let flags = read_unsignedbyte(buf);

        Box::new(EntityEffectPacket {
            entity_id,
            effect_id,
            amplifier,
            duration,
            flags,
        })
    }
}
