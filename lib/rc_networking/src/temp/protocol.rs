use crate::protocol::clientbound::block_update::BlockUpdate;
use crate::protocol::clientbound::chat::ChatSent;
use crate::protocol::clientbound::chunk_update::FullChunkUpdate;
use crate::protocol::serverbound::player_move::PlayerMove;
use crate::protocol::serverbound::player_rotate::PlayerRotate;
use crate::protocol::clientbound::entity_moved::EntityMoved;
use crate::protocol::clientbound::spawn_entity::SpawnEntity;
use crate::protocol::clientbound::entity_rotated::EntityRotated;
use crate::protocol::clientbound::despawn_entity::DespawnEntity;
use serde::{Serialize, Deserialize};

pub mod clientbound;
pub mod serverbound;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(C)]
pub enum Protocol {
    PlayerMove(PlayerMove),
    EntityMoved(EntityMoved),
    PlayerRotate(PlayerRotate),
    EntityRotated(EntityRotated),
    DespawnEntity(DespawnEntity),
    BlockUpdate(BlockUpdate),
    ChatSent(ChatSent),
    PartialChunkUpdate(FullChunkUpdate),
    SpawnEntity(SpawnEntity),
}