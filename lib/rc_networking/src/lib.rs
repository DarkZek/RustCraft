#![allow(dead_code)]
#![feature(trivial_bounds)]

pub mod client;
pub mod events;
pub mod protocol;
pub mod types;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;
pub mod bistream;
#[cfg(not(target_arch = "wasm32"))]
mod skip_verification;

use protocol::Protocol;

// TODO: Remove panics and handle all errors properly

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
        | Protocol::GameObjectMoved(_)
        | Protocol::PlayerRotate(_)
        | Protocol::GameObjectRotated(_) => Channel::Unreliable,

        Protocol::BlockUpdate(_)
        | Protocol::Disconnect(_)
        | Protocol::ChatSent(_)
        | Protocol::PlayerChat(_)
        | Protocol::DespawnGameObject(_)
        | Protocol::SpawnGameObject(_)
        | Protocol::UpdateLoading(_)
        | Protocol::RequestChunk(_)
        | Protocol::ServerState(_)
        | Protocol::UpdateInventorySlot(_)
        | Protocol::UpdateInventory(_)
        | Protocol::Authorization(_)
        | Protocol::AuthorizationAccepted
        | Protocol::PlaceBlock(_)
        | Protocol::DestroyBlock(_)
        | Protocol::ChangeHotbarSlot(_)
        | Protocol::AcknowledgeChunk(_) => Channel::Reliable,

        Protocol::FullChunkUpdate(_) | Protocol::PartialChunkUpdate(_) => Channel::Chunk,
    }
}
