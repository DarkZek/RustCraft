pub mod client;
pub mod events;
pub mod protocol;
pub mod types;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;
#[cfg(not(target_arch = "wasm32"))]
pub mod bistream;

use protocol::Protocol;
use std::net::SocketAddr;
use std::time::Duration;

#[derive(Copy, Clone, Debug)]
pub enum Channel {
    Reliable,
    Unreliable,
    Chunk,
}

impl From<Channel> for u8 {
    fn from(value: Channel) -> Self {
        match value {
            Channel::Reliable => 0,
            Channel::Unreliable => 1,
            Channel::Chunk => 2,
        }
    }
}

fn get_channel(protocol: &Protocol) -> Channel {
    match protocol {
        Protocol::PlayerMove(_)
        | Protocol::EntityMoved(_)
        | Protocol::PlayerRotate(_)
        | Protocol::EntityRotated(_) => Channel::Unreliable,

        Protocol::BlockUpdate(_)
        | Protocol::ChatSent(_)
        | Protocol::DespawnGameObject(_)
        | Protocol::SpawnGameObject(_)
        | Protocol::UpdateLoading(_)
        | Protocol::RequestChunk(_)
        | Protocol::ServerState(_)
        | Protocol::UpdateInventorySlot(_)
        | Protocol::UpdateInventory(_)
        | Protocol::AcknowledgeChunk(_) => Channel::Reliable,

        Protocol::FullChunkUpdate(_) | Protocol::PartialChunkUpdate(_) => Channel::Chunk,
    }
}
