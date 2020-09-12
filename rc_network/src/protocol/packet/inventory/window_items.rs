use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_unsignedbyte, read_bool, read_int, read_short};
use std::io::{Cursor, Seek, Read};
use crate::protocol::types::slot::Slot;

// https://wiki.vg/Protocol#Window_Items

#[derive(Debug)]
pub struct WindowItemsPacket {
    pub window_id: u8,
    pub slot: Vec<Slot>
}

impl PacketType for WindowItemsPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let window_id = read_unsignedbyte(buf);
        let slot_len = read_short(buf);
        let mut slot = Vec::new();

        for _ in 0..slot_len {
            slot.push(Slot::deserialize(buf));
        }

        Box::new(WindowItemsPacket {
            window_id,
            slot
        })
    }
}
