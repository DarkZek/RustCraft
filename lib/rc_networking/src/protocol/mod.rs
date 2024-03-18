use crate::protocol::clientbound::block_update::BlockUpdate;
use crate::protocol::clientbound::chat::ChatSent;
use crate::protocol::clientbound::chunk_update::{FullChunkUpdate, PartialChunkUpdate};
use crate::protocol::clientbound::despawn_game_object::DespawnGameObject;
use crate::protocol::clientbound::entity_moved::EntityMoved;
use crate::protocol::clientbound::entity_rotated::EntityRotated;
use crate::protocol::clientbound::server_state::ServerState;
use crate::protocol::clientbound::spawn_game_object::SpawnGameObject;
use crate::protocol::clientbound::update_loading::UpdateLoading;
use crate::protocol::serverbound::acknowledge_chunk::AcknowledgeChunk;
use crate::protocol::serverbound::player_move::PlayerMove;
use crate::protocol::serverbound::player_rotate::PlayerRotate;
use crate::protocol::serverbound::request_chunk::RequestChunk;
use self::clientbound::update_inventory::UpdateInventory;
use self::clientbound::update_inventory_slot::UpdateInventorySlot;
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
    DespawnGameObject(DespawnGameObject),
    BlockUpdate(BlockUpdate),
    ChatSent(ChatSent),
    ServerState(ServerState),
    // Unused as networking solution does not support such large packets being sent so fast
    FullChunkUpdate(FullChunkUpdate),
    PartialChunkUpdate(PartialChunkUpdate),
    SpawnGameObject(SpawnGameObject),
    RequestChunk(RequestChunk),
    UpdateLoading(UpdateLoading),
    AcknowledgeChunk(AcknowledgeChunk),
    UpdateInventorySlot(UpdateInventorySlot),
    UpdateInventory(UpdateInventory)
}
