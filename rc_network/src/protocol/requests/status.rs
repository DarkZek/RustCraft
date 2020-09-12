use crate::protocol::types::{PVarType, PVarTypeTemplate};
use crate::protocol::data::write_types::{write_string, write_ushort, write_varint, write_varlong};
use crate::stream::NetworkStream;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::protocol::data::writer::PacketBuilder;
use crate::protocol::data::reader::PacketReader;

pub struct PingRequest;

impl PingRequest {
    pub fn send(connection_host: String, connection_port: u32) -> String {
        let mut stream =
            NetworkStream::connect(format!("{}:{}", connection_host, connection_port)).unwrap();

        // Handshake packet
        let mut packet = PacketBuilder::new(0x0);

        // Protocol version
        write_varint(736, &mut packet.data);
        // Connecting address
        write_string("localhost", &mut packet.data);
        // Port
        write_ushort(25565, &mut packet.data);
        // Next state
        write_varint(1, &mut packet.data);

        packet.send(&mut stream);

        // Status packet
        let mut packet = PacketBuilder::new(0x0);

        packet.send(&mut stream);

        let response = PacketReader::new()
            .add_token(PVarTypeTemplate::String)
            .read(&mut stream);

        // Send ping packet
        let mut packet = PacketBuilder::new(0x01);

        write_varlong(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i128,
            &mut packet.data,
        );

        packet.send(&mut stream);

        match response.tokens.get(0).unwrap() {
            PVarType::String(str) => return str.clone(),
            _ => panic!("Unexpected returned variable"),
        }
    }
}
