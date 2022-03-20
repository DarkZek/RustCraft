use crate::protocol::data::read_types::{read_short, read_unsignedbyte};
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use crate::protocol::types::slot::Slot;
use std::io::Cursor;

// https://wiki.vg/Protocol#Window_Items

#[derive(Debug)]
pub struct WindowItemsPacket {
    pub window_id: u8,
    pub slot: Vec<Slot>,
}

impl ClientBoundPacketType for WindowItemsPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let window_id = read_unsignedbyte(buf);
        let slot_len = read_short(buf);
        let mut slot = Vec::new();

        for _ in 0..slot_len {
            slot.push(Slot::deserialize(buf));
        }

        Box::new(WindowItemsPacket { window_id, slot })
    }
}
