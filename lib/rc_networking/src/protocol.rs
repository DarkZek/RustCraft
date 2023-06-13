use crate::protocol::clientbound::block_update::BlockUpdate;
use crate::protocol::clientbound::chat::ChatSent;
use crate::protocol::clientbound::chunk_update::{FullChunkUpdate, PartialChunkUpdate};
use crate::protocol::clientbound::despawn_entity::DespawnEntity;
use crate::protocol::clientbound::entity_moved::EntityMoved;
use crate::protocol::clientbound::entity_rotated::EntityRotated;
use crate::protocol::clientbound::spawn_entity::SpawnEntity;
use crate::protocol::clientbound::update_loading::UpdateLoading;
use crate::protocol::serverbound::player_move::PlayerMove;
use crate::protocol::serverbound::player_rotate::PlayerRotate;
use crate::protocol::serverbound::request_chunk::RequestChunk;
use serde::{Deserialize, Serialize};

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
    // Unused as networking solution does not support such large packets being sent so fast
    FullChunkUpdate(FullChunkUpdate),
    PartialChunkUpdate(PartialChunkUpdate),
    SpawnEntity(SpawnEntity),
    RequestChunk(RequestChunk),
    UpdateLoading(UpdateLoading),
}
