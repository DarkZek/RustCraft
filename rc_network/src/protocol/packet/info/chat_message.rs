use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_unsignedbyte, read_string};
use std::io::{Cursor};

#[derive(Debug)]
pub struct ChatMessagePacket {
    pub json_data: String,
    pub position: u8
}

impl PacketType for ChatMessagePacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let json_data = read_string(buf);
        let position = read_unsignedbyte(buf);

        Box::new(ChatMessagePacket {
            json_data,
            position
        })
    }
}
