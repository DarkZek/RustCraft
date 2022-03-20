use crate::protocol::data::read_types::{read_double, read_float, read_unsignedbyte, read_varint};
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct LoginSuccessPacket {
    pub uuid: String,
    pub username: String,
}

impl ClientBoundPacketType for LoginSuccessPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let mut uuid = Vec::with_capacity(36);
        let mut username = Vec::with_capacity(16);
        buf.read_exact(&mut uuid).unwrap();
        buf.read_exact(&mut username).unwrap();

        Box::new(LoginSuccessPacket {
            uuid: String::from_utf8(uuid).unwrap(),
            username: String::from_utf8(username).unwrap(),
        })
    }
}
