use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::{Shutdown};
use crate::protocol::Protocol;
use mio::net::TcpStream;
use crate::error::ProtocolError;

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

        self.stream.flush();

        Ok(())
    }

    pub fn read_packet(&mut self) -> Result<Protocol, ProtocolError> {
        let mut data = vec![0u8; 4]; // 4 Is the size of u32
        self.stream.read_exact(&mut data)?;

        let packet_len: u32 = bincode::deserialize(&data[..])?;

        data = vec![0u8; packet_len as usize];

        self.stream.read_exact(&mut data)?;

        let packet: Protocol = bincode::deserialize(&data[..])?;

        Ok(packet)
    }

    pub fn close(&self) {
        self.stream.shutdown(Shutdown::Both);
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}