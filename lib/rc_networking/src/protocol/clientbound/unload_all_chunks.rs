use rc_shared::constants::GameObjectId;
use rc_shared::game_objects::GameObjectData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(C)]
pub struct UnloadAllChunks {
    // Unused
    pub flag: bool
}
