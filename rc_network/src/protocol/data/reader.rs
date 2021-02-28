use crate::protocol::data::read_types::{length_as_varint, read_string, read_varint};
use crate::protocol::data::Packet;
use crate::protocol::types::{PVarType, PVarTypeTemplate};
use crate::stream::NetworkStream;
use std::io::Read;

pub struct PacketReader {
    tokens: Vec<PVarTypeTemplate>,
}

impl PacketReader {
    pub fn new() -> PacketReader {
        PacketReader { tokens: vec![] }
    }

    pub fn add_token(mut self, token: PVarTypeTemplate) -> PacketReader {
        self.tokens.push(token);
        self
    }

    pub fn read(self, stream: &mut NetworkStream) -> Packet {
        // Get size of packet
        let len = read_varint(stream);
        let packet_id = read_varint(stream);

        let _remaining_len = len - length_as_varint(packet_id);

        let mut tokens = Vec::with_capacity(self.tokens.len());

        for token in self.tokens {
            // Parse token
            match token {
                PVarTypeTemplate::VarInt => {
                    let int = read_varint(stream);
                    tokens.push(PVarType::VarInt(int));
                }
                PVarTypeTemplate::String => {
                    let string = read_string(stream);
                    tokens.push(PVarType::String(string));
                }
                PVarTypeTemplate::VarIntByteArray => {
                    let len = read_varint(stream);

                    let mut data = vec![0; len as usize];
                    stream.read_exact(&mut data).unwrap();

                    tokens.push(PVarType::VarIntByteArray(data));
                }
                _ => unimplemented!(),
            }
        }

        Packet {
            id: packet_id as u16,
            len: len as u32,
            tokens,
        }
    }
}
