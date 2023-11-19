use crate::game::blocks::loading::BlockStatesFile;
use crate::game::blocks::{Block, LootTableEntry};
use bevy::log::warn;
use bevy::prelude::Handle;
use bevy::prelude::Resource;
use bevy::reflect::TypeUuid;

#[derive(Debug, Clone, TypeUuid, Resource)]
#[uuid = "97103fab-1e50-36b7-0c33-0938a62b0809"]
pub struct BlockStates {
    pub states: Vec<Block>,
    pub loot_tables: Vec<Vec<LootTableEntry>>,
    /// Used to tell the blockstates to recalculate, only used when the blockstates are ready but waiting on the texture atlas to finish loading.rs
    pub recalculate: bool,
    pub asset: Option<Handle<BlockStatesFile>>,
}

impl BlockStates {
    pub fn new() -> BlockStates {
        BlockStates {
            states: vec![],
            loot_tables: vec![],
            recalculate: false,
            asset: None,
        }
    }

    // Possibly remove, keeping it because it was in old version and I might need it
    pub fn get_block(&self, i: usize) -> &Block {
        if let Some(val) = self.states.get(i) {
            val
        } else {
            warn!("Invalid block state received: {}", i);
            self.states.get(0).unwrap()
        }
    }
}
