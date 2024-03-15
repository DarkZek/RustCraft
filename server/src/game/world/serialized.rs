use crate::game::chunk::ChunkData;
use crate::game::game_object::GameObject;
use crate::game::transform::Transform;
use rc_shared::constants::GameObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DeserializedChunkData {
    pub version: u32,
    pub data: ChunkData,
    pub game_objects: Vec<(GameObjectId, GameObject, Transform)>,
}
