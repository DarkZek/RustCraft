use crate::protocol::read_types::{length_as_varint, read_string, read_varint};
use crate::protocol::types::{PVarType, PVarTypeTemplate};
use crate::protocol::write_types::write_varint;
use crate::stream::NetworkStream;
use std::io::{Read, Write};

pub struct PacketBuilder {
    id: u16,
    pub data: Vec<u8>,
}

impl PacketBuilder {
    pub fn new(id: u16) -> PacketBuilder {
        PacketBuilder {
            id,
            data: Vec::new(),
        }
    }

    pub fn send(&mut self, stream: &mut NetworkStream) {
        let mut packet_id = Vec::new();
        write_varint(self.id as i32, &mut packet_id);

        // Calculate packet Length
        let len = packet_id.len() + self.data.len();

        // Final data transmission
        let mut data_stream = Vec::new();
        write_varint(len as i32, &mut data_stream);

        // Add packet id
        data_stream.append(&mut packet_id);

        // Add data
        data_stream.append(&mut self.data);

        stream.write_all(&mut data_stream);
        stream.flush();
    }
}

pub struct Packet {
    pub id: u16,
    pub len: u32,
    pub tokens: Vec<PVarType>,
}

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

        let remaining_len = len - length_as_varint(packet_id);

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
                    stream.read_exact(&mut data);

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
