use crate::protocol::data::read_types::read_long;
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct KeepAlivePacket {
    pub keep_alive_id: i64,
}

impl ClientBoundPacketType for KeepAlivePacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let keep_alive_id = read_long(buf);

        Box::new(KeepAlivePacket { keep_alive_id })
    }
}
