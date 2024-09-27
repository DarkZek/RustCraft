use crate::protocol::clientbound::block_update::BlockUpdate;
use crate::protocol::clientbound::chat::ChatSent;
use crate::protocol::clientbound::chunk_update::{FullChunkUpdate, PartialChunkUpdate};
use crate::protocol::clientbound::despawn_game_object::DespawnGameObject;
use crate::protocol::clientbound::game_object_moved::GameObjectMoved;
use crate::protocol::clientbound::game_object_rotated::GameObjectRotated;
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
use crate::protocol::serverbound::change_hotbar_slot::ChangeHotbarSlot;
use crate::protocol::serverbound::destroy_block::DestroyBlock;
use crate::protocol::serverbound::place_block::PlaceBlock;
use crate::protocol::serverbound::player_chat::PlayerChat;

pub mod clientbound;
pub mod serverbound;

/// The HTTP/3 ALPN is required when negotiating a QUIC connection.
pub const ALPN: &[u8] = b"h3";

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(C)]
pub enum Protocol {
    Authorization(String),
    AuthorizationAccepted,
    PlayerMove(PlayerMove),
    GameObjectMoved(GameObjectMoved),
    PlayerRotate(PlayerRotate),
    GameObjectRotated(GameObjectRotated),
    DespawnGameObject(DespawnGameObject),
    BlockUpdate(BlockUpdate),
    ChatSent(ChatSent),
    ServerState(ServerState),
    PlayerChat(PlayerChat),
    PlaceBlock(PlaceBlock),
    ChangeHotbarSlot(ChangeHotbarSlot),
    DestroyBlock(DestroyBlock),
    // Unused as networking solution does not support such large packets being sent so fast
    FullChunkUpdate(FullChunkUpdate),
    PartialChunkUpdate(PartialChunkUpdate),
    SpawnGameObject(SpawnGameObject),
    RequestChunk(RequestChunk),
    UpdateLoading(UpdateLoading),
    AcknowledgeChunk(AcknowledgeChunk),
    UpdateInventorySlot(UpdateInventorySlot),
    UpdateInventory(UpdateInventory),
    Disconnect(String)
}
