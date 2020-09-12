use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_string};
use std::io::{Cursor, Seek, Read};

#[derive(Debug)]
pub struct PluginMessagePacket {
    pub channel: String,
    pub data: Vec<u8>
}

impl PacketType for PluginMessagePacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let channel = read_string(buf);
        let mut data = Vec::new();
        buf.read_to_end(&mut data);

        Box::new(PluginMessagePacket {
            channel,
            data
        })
    }
}
