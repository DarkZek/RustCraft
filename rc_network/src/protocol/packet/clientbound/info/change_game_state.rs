use crate::protocol::data::read_types::{read_float, read_unsignedbyte};
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use std::io::Cursor;

// https://wiki.vg/Protocol#Change_Game_state

#[derive(Debug)]
pub struct ChangeGameStatePacket {
    pub reason: u8,
    pub value: f32,
}

impl ClientBoundPacketType for ChangeGameStatePacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let reason = read_unsignedbyte(buf);
        let value = read_float(buf);

        Box::new(ChangeGameStatePacket { reason, value })
    }
}
