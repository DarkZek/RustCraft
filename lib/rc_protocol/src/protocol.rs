use crate::protocol::clientbound::block_update::BlockUpdate;
use crate::protocol::clientbound::chat::ChatSent;
use crate::protocol::clientbound::chunk_update::FullChunkUpdate;
use crate::protocol::serverbound::player_move::PlayerMove;
use crate::protocol::serverbound::player_rotate::PlayerRotate;
use serde::{Serialize, Deserialize};
use crate::protocol::clientbound::entity_moved::EntityMoved;
use crate::protocol::clientbound::spawn_entity::SpawnEntity;
use crate::protocol::clientbound::entity_rotated::EntityRotated;
use crate::protocol::clientbound::despawn_entity::DespawnEntity;

pub mod clientbound;
pub mod serverbound;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(C)]
pub enum Protocol {
    //Ping(Ping), // unused
    //Pong(Pong), // unused
    //PlayerJoin(PlayerJoin), // unused
    PlayerMove(PlayerMove),
    EntityMoved(EntityMoved),
    PlayerRotate(PlayerRotate),
    EntityRotated(EntityRotated),
    DespawnEntity(DespawnEntity),
    //PlayerLeave(PlayerLeave), // unused
    BlockUpdate(BlockUpdate),
    ChatSent(ChatSent),
    PartialChunkUpdate(FullChunkUpdate),
    //UserAuthenticate(UserAuthenticate), // unused
    SpawnEntity(SpawnEntity),
    //Disconnect(Disconnect) // unused
}