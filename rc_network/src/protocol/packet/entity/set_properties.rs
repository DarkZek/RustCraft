use crate::protocol::data::read_types::{read_double, read_int, read_string, read_varint};
use crate::protocol::packet::PacketType;
use crate::protocol::types::modifier_data::ModifierData;
use std::io::Cursor;

#[derive(Debug)]
pub struct EntitySetPropertiesPacket {
    pub entity_id: i32,
    pub properties: Vec<(String, f64, Vec<ModifierData>)>,
}

impl PacketType for EntitySetPropertiesPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_varint(buf);
        let properties_len = read_int(buf);
        let mut properties = Vec::new();

        for _ in 0..properties_len {
            let key = read_string(buf);
            let value = read_double(buf);
            let modifiers_len = read_varint(buf);
            let mut modifiers = Vec::new();

            for _ in 0..modifiers_len {
                modifiers.push(ModifierData::deserialize(buf));
            }

            properties.push((key, value, modifiers));
        }

        Box::new(EntitySetPropertiesPacket {
            entity_id,
            properties,
        })
    }
}
