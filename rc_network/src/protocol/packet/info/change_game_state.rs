use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_unsignedbyte, read_bool, read_int, read_float};
use std::io::{Cursor, Seek, Read};

// https://wiki.vg/Protocol#Change_Game_state

#[derive(Debug)]
pub struct ChangeGameStatePacket {
    pub reason: u8,
    pub value: f32
}

impl PacketType for ChangeGameStatePacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let reason = read_unsignedbyte(buf);
        let value = read_float(buf);

        Box::new(ChangeGameStatePacket {
            reason,
            value
        })
    }
}
