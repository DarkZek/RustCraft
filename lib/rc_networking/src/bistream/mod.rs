
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

use bevy::log::debug;
use web_transport::{Error, RecvStream};
#[cfg(not(target_arch = "wasm32"))]
pub use native::BiStream;
#[cfg(target_arch = "wasm32")]
pub use wasm::BiStream;

#[derive(Debug, Clone, Copy)]
pub enum StreamError {
    Error,
}

async fn read_exact(recv: &mut RecvStream, len: usize) -> Result<Vec<u8>, Error> {
    // TODO: Remove copying here
    let mut chunk_data = Vec::new();
    while chunk_data.len() < len {
        let remaining_len = len - chunk_data.len();

        debug!("[Task Pool] [Reader] Remaining length {}", remaining_len);

        let mut data = recv.read(remaining_len).await?.unwrap().to_vec();

        chunk_data.append(&mut data);
    }

    Ok(chunk_data)
}