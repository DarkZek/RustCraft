use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_long};
use std::io::{Cursor, Seek, Read};

#[derive(Debug)]
pub struct KeepAlivePacket {
    pub keep_alive_id: i64
}

impl PacketType for KeepAlivePacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let keep_alive_id = read_long(buf);

        Box::new(KeepAlivePacket {
            keep_alive_id
        })
    }
}
