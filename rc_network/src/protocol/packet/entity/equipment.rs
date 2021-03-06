use crate::protocol::data::read_types::read_varint;
use crate::protocol::packet::PacketType;
use crate::protocol::types::slot::Slot;
use std::io::Cursor;

#[derive(Debug)]
pub struct EntityEquipmentPacket {
    pub entity_id: i64,
    // 0: src hand, 1: off hand, 2–5: armor slot (2: boots, 3: leggings, 4: chestplate, 5: helmet)
    pub slot: i64,
    pub item: Slot,
}

impl PacketType for EntityEquipmentPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_varint(buf);
        let slot = read_varint(buf);
        let item = Slot::deserialize(buf);

        Box::new(EntityEquipmentPacket {
            entity_id,
            slot,
            item,
        })
    }
}
