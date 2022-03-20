use crate::protocol::data::read_types::read_unsignedbyte;
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct HeldItemChangePacket {
    pub slot: u8,
}

impl ClientBoundPacketType for HeldItemChangePacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let slot = read_unsignedbyte(buf);

        Box::new(HeldItemChangePacket { slot })
    }
}
