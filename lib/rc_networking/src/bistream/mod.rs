
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

use std::fmt::{Display, Formatter};
use bevy::log::debug;
use bevy::prelude::{info, warn};
use thiserror::Error;
use web_transport::{Error, RecvStream, SendStream};
#[cfg(not(target_arch = "wasm32"))]
pub use native::BiStream;
#[cfg(target_arch = "wasm32")]
pub use wasm::BiStream;
use crate::protocol::Protocol;

#[derive(Debug, Clone, Copy, Error)]
pub enum StreamError {
    Error,
    StreamClosed,
    MalformedPacket
}

impl Display for StreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Reads an exact amount of bytes from a RecvStream
async fn read_exact(recv: &mut RecvStream, len: usize) -> Result<Vec<u8>, anyhow::Error> {
    // TODO: Remove copying here
    let mut chunk_data = Vec::new();
    while chunk_data.len() < len {
        let remaining_len = len - chunk_data.len();

        debug!("[Task Pool] [Reader] Remaining length {}", remaining_len);

        let Ok(Some(mut data)) = recv.read(remaining_len).await else {
            return Err(StreamError::StreamClosed.into());
        };

        chunk_data.append(&mut data.to_vec());
    }

    Ok(chunk_data)
}

/// Writes a `Protocol` to some RecvStream
pub async fn send_protocol(packet: &Protocol, send: &mut SendStream) -> Result<(), anyhow::Error> {
    let packet_data = bincode::serialize(&packet).unwrap();

    debug!("[Task Pool] [Writer] Sending packet length {}", packet_data.len());

    if let Err(e) = send.write(bincode::serialize(&(packet_data.len() as u32)).unwrap().as_slice()).await {
        warn!("Failed writing to stream: {:?}", e);
        return Err(StreamError::Error.into());
    }
    if let Err(e) = send.write(&*packet_data).await {
        warn!("Failed writing to stream: {:?}", e);
        return Err(StreamError::Error.into());
    }

    Ok(())
}

/// Reads a `Protocol` from some RecvStream
pub async fn recv_protocol(recv: &mut RecvStream) -> Result<Protocol, anyhow::Error> {
    let len_data = read_exact(recv, size_of::<u32>()).await?;

    let len = match bincode::deserialize::<u32>(&len_data) {
        Ok(v) => v,
        Err(_) => return Err(StreamError::MalformedPacket.into())
    };

    debug!("[Task Pool] [Reader] Received data length {}", len);

    let chunk_data = read_exact(recv, len as usize).await?;

    assert_eq!(chunk_data.len(), len as usize, "Chunk data was not equal for packet with length. Read: {} Expected: {}", chunk_data.len(), len);

    let data = match bincode::deserialize::<Protocol>(&chunk_data) {
        Ok(v) => v,
        Err(_) => return Err(StreamError::MalformedPacket.into())
    };

    Ok(data)
}