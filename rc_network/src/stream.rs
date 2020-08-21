use std::io::Result;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

/// A TcpStream wrapper that counts bytes read
pub struct NetworkStream {
    stream: TcpStream,
    byte_counter: u64,
}

impl NetworkStream {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<NetworkStream> {
        let stream = TcpStream::connect(addr)?;

        Ok(NetworkStream {
            stream,
            byte_counter: 0,
        })
    }

    pub fn reset_byte_counter(&mut self) {
        self.byte_counter = 0;
    }

    pub fn get_bytes_read(&self) -> u64 {
        self.byte_counter
    }
}

impl Read for NetworkStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.byte_counter += buf.len() as u64;
        self.stream.read(buf)
    }
}

impl Write for NetworkStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.stream.flush()
    }
}
