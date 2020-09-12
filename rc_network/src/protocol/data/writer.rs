use crate::stream::NetworkStream;
use crate::protocol::data::write_types::write_varint;
use std::io::Write;

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