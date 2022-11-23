use crate::protocol::clientbound::block_update::BlockUpdate;
use crate::protocol::clientbound::chat::ChatSent;
use crate::protocol::clientbound::chunk_update::FullChunkUpdate;
use crate::protocol::clientbound::ping::Ping;
use crate::protocol::clientbound::player_join::PlayerJoin;
use crate::protocol::clientbound::player_leave::PlayerLeave;
use crate::protocol::serverbound::authenticate::UserAuthenticate;
use crate::protocol::serverbound::player_move::PlayerMove;
use crate::protocol::serverbound::player_rotate::PlayerRotate;
use crate::protocol::serverbound::pong::Pong;
use serde::{Serialize, Deserialize};
use crate::protocol::clientbound::entity_moved::EntityMoved;
use crate::protocol::clientbound::spawn_entity::SpawnEntity;
use crate::protocol::clientbound::entity_rotated::EntityRotated;
use crate::protocol::serverbound::disconnect::Disconnect;
use crate::protocol::clientbound::despawn_entity::DespawnEntity;

pub mod clientbound;
pub mod serverbound;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(C)]
pub enum Protocol {
    Ping(Ping),
    Pong(Pong),
    PlayerJoin(PlayerJoin),
    PlayerMove(PlayerMove),
    EntityMoved(EntityMoved),
    PlayerRotate(PlayerRotate),
    EntityRotated(EntityRotated),
    DespawnEntity(DespawnEntity),
    PlayerLeave(PlayerLeave),
    BlockUpdate(BlockUpdate),
    ChatSent(ChatSent),
    PartialChunkUpdate(FullChunkUpdate),
    UserAuthenticate(UserAuthenticate),
    SpawnEntity(SpawnEntity),
    Disconnect(Disconnect)
}