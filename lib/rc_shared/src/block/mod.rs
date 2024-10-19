pub mod face;
pub mod types;
pub mod blocks;
mod uid;

use crate::atlas::TextureAtlasTrait;
use crate::block::types::{Block, LootTableEntry};
use bevy::log::warn;
use bevy::prelude::{App, AssetApp, AssetServer, Handle, Plugin, Resource, Startup, Update};
use bevy::reflect::TypePath;
use std::sync::OnceLock;
use crate::block::blocks::get_blocks;

pub struct BlockStatesPlugin;

impl Plugin for BlockStatesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BlockStates::new());
    }
}

#[derive(Debug, Clone, TypePath, Resource)]
pub struct BlockStates {
    pub block_index: Vec<usize>,
    pub states: Vec<Block>,
    /// Used to recalculate type mapping from identifier to index when items list is updated
    pub recalculate_items: bool,
}

impl BlockStates {
    pub fn new() -> BlockStates {
        BlockStates {
            block_index: vec![],
            states: vec![],
            recalculate_items: false,
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

    pub fn get_by_id(&self, name: &str) -> Option<(usize, &Block)> {
        for (i, state) in self.states.iter().enumerate() {
            if state.identifier == name {
                return Some((i, state));
            }
        }

        None
    }

    pub fn calculate_states(&mut self) {
        self.states.clear();

        for (i, block) in get_blocks().iter().enumerate() {
            let mut blocks = (block.get_variants)();
            self.block_index.append(&mut vec![i; blocks.len()]);
            self.states.append(&mut blocks);
        }
    }
}
