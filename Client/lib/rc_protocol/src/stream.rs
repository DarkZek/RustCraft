use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown};
use crate::protocol::Protocol;
use crate::error::ProtocolError;
use std::net::TcpStream;
use byteorder::{LittleEndian, ReadBytesExt};

/// Tool to write and read packets to channel
pub struct GameStream {
    pub stream: TcpStream
}

impl GameStream {
    pub fn new(stream: TcpStream) -> GameStream {
        GameStream {
            stream
        }
    }

    pub fn write_packet(&mut self, packet: &Protocol) -> io::Result<()> {
        let packet = bincode::serialize(&packet).unwrap();
        // Just size for now
        let header: u32 = packet.len() as u32;

        self.stream.write_all(&bincode::serialize(&header).unwrap())?;
        self.stream.write_all(&packet)?;

        self.stream.flush().unwrap();

        Ok(())
    }

    pub fn read_packet(&mut self) -> Result<Protocol, ProtocolError> {
        let packet_len = self.stream.read_u32::<LittleEndian>()?;

        let mut data = vec![0u8; packet_len as usize];

        self.stream.read_exact(&mut data)?;

        //let t = self.stream.read_exact(&mut data)?;

        let packet: Protocol = bincode::deserialize_from(&self.stream)?;

        Ok(packet)
    }

    #[allow(unused_must_use)]
    pub fn close(&self) {
        self.stream.shutdown(Shutdown::Both);
    }
}