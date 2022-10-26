use crate::protocol::clientbound::block_update::BlockUpdate;
use crate::protocol::clientbound::chat::ChatSent;
use crate::protocol::clientbound::chunk_update::PartialChunkUpdate;
use crate::protocol::clientbound::ping::Ping;
use crate::protocol::clientbound::player_join::PlayerJoin;
use crate::protocol::clientbound::player_leave::PlayerLeave;
use crate::protocol::clientbound::player_moved::PlayerMoved;
use crate::protocol::clientbound::player_rotated::PlayerRotated;
use crate::protocol::serverbound::authenticate::UserAuthenticate;
use crate::protocol::serverbound::player_move::PlayerMove;
use crate::protocol::serverbound::player_rotate::PlayerRotate;
use crate::protocol::serverbound::pong::Pong;
use serde::{Serialize, Deserialize};
use crate::constants::UserId;

pub mod clientbound;
pub mod serverbound;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Protocol {
    Ping(Ping),
    Pong(Pong),
    PlayerJoin(PlayerJoin),
    PlayerMove(PlayerMove),
    PlayerMoved(PlayerMoved),
    PlayerRotate(PlayerRotate),
    PlayerRotated(PlayerRotated),
    PlayerLeave(PlayerLeave),
    BlockUpdate(BlockUpdate),
    ChatSent(ChatSent),
    PartialChunkUpdate(PartialChunkUpdate),
    UserAuthenticate(UserAuthenticate),
}

/// Alias used to differentiate the packets for use with Bevy's ECS Event Readers
pub struct SendPacket(pub Protocol);

impl std::ops::Deref for SendPacket {
    type Target = Protocol;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Alias used to differentiate the packets for use with Bevy's ECS Event Readers
pub struct ReceivePacket(pub Protocol, pub UserId);

impl std::ops::Deref for ReceivePacket {
    type Target = Protocol;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}