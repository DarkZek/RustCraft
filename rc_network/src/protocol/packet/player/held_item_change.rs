use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_unsignedbyte, read_bool, read_float};
use std::io::{Cursor, Seek, Read};

#[derive(Debug)]
pub struct HeldItemChangePacket {
    pub slot: u8
}

impl PacketType for HeldItemChangePacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let slot = read_unsignedbyte(buf);

        Box::new(HeldItemChangePacket {
            slot
        })
    }
}
