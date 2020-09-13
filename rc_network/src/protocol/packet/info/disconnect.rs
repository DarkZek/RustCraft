use crate::protocol::data::read_types::{read_long, read_string};
use crate::protocol::packet::PacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct DisconnectPacket {
    pub reason: String,
}

impl PacketType for DisconnectPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let reason = read_string(buf);

        Box::new(DisconnectPacket { reason })
    }
}
