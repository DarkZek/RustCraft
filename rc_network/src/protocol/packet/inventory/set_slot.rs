use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_unsignedbyte, read_short};
use std::io::{Cursor};
use crate::protocol::types::slot::Slot;

#[derive(Debug)]
pub struct SetSlotPacket {
    pub window_id: u8,
    pub slot: i16,
    pub slot_data: Slot
}

impl PacketType for SetSlotPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let window_id = read_unsignedbyte(buf);
        let slot = read_short(buf);
        let slot_data = Slot::deserialize(buf);

        Box::new(SetSlotPacket {
            window_id,
            slot,
            slot_data
        })
    }
}
