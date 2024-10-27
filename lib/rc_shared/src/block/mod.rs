pub mod face;
pub mod types;
pub mod blocks;
mod uid;
pub mod definition;
mod bench;

use std::ops::Deref;
use crate::atlas::TextureAtlasTrait;
use crate::block::types::{VisualBlock, LootTableEntry};
use bevy::log::warn;
use bevy::prelude::{App, AssetApp, AssetServer, Handle, Plugin, Resource, Startup, Update};
use bevy::reflect::TypePath;
use std::sync::OnceLock;
use serde::{Deserialize, Serialize};
use crate::block::definition::{BLOCK_DEFINITIONS, BlockDefinition, set_blocks};

pub struct BlockStatesPlugin;

impl Plugin for BlockStatesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BlockStates::new());
    }
}

/// The index of a block definition
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockDefinitionIndex(pub(crate) usize);

impl Deref for BlockDefinitionIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A unique id of a single block variant over all blocks
pub type BlockId = u32;

#[derive(Debug, Clone, TypePath, Resource)]
pub struct BlockStates {
    // A lookup table from a block index, to a block definition index & block uid
    pub(crate) block_index: Vec<(BlockDefinitionIndex, BlockId)>,
    // A lookup table from a block definition index, to a block id
    pub(crate) block_id: Vec<BlockId>,
    // Visual block lookups are a very hot path, so this caches visual blocks
    pub visual_block_cache: Vec<Option<Box<VisualBlock>>>,
    /// Used to recalculate type mapping from identifier to index when items list is updated
    pub recalculate_items: bool,
}

impl BlockStates {
    pub fn new() -> BlockStates {
        BlockStates {
            block_index: vec![],
            block_id: vec![],
            visual_block_cache: vec![],
            recalculate_items: false,
        }
    }

    pub fn get_block_from_id(&self, block_index: BlockId) -> WorldBlock {
        if let Some((index, block_id)) = self.block_index.get(block_index as usize) {
            let definition = BLOCK_DEFINITIONS.get().unwrap().get(index.0).unwrap();

            WorldBlock {
                definition,
                block_id: *block_id
            }
        } else {
            warn!("Invalid block index received: {:?}", block_index);
            let definition = BLOCK_DEFINITIONS.get().unwrap().get(0).unwrap();

            WorldBlock {
                definition,
                block_id: 0
            }
        }
    }

    pub fn get_by_identifier(&self, name: &str) -> Option<(usize, WorldBlock)> {
        for (i, definition) in BLOCK_DEFINITIONS.get().unwrap().iter().enumerate() {
            if definition.identifier == name {
                let block = WorldBlock {
                    definition,
                    block_id: 0
                };
                return Some((i, block));
            }
        }

        None
    }

    /// Gets the beginning of the block id's for a block definition
    pub fn get_id_by_definition(&self, block_definition_index: BlockDefinitionIndex) -> Option<BlockId> {
        self.block_id.get(block_definition_index.0).map(|t| *t)
    }

    /// Gets the beginning of the block id's for a block definition
    pub fn get_definition_index_by_identifier(identifier: &str) -> Option<BlockDefinitionIndex> {
        BLOCK_DEFINITIONS.get().unwrap()
            .iter()
            .enumerate()
            .find(|(i, definition)| definition.identifier == identifier)
            .map(|(i, _)| BlockDefinitionIndex(i))
    }

    /// Gets the beginning of the block id's for a block definition
    pub fn get_definition_by_index(index: BlockDefinitionIndex) -> Option<&'static BlockDefinition> {
        BLOCK_DEFINITIONS.get().unwrap()
            .get(index.0)
    }

    /// Gets the beginning of the block id's for a block definition
    pub fn get_definition_index_by_id(&self, id: BlockId) -> Option<BlockDefinitionIndex> {
        self.block_index.get(id as usize).map(|(index, _)| *index)
    }

    pub fn calculate_states(&mut self) {
        set_blocks();
        self.block_index.clear();

        for (i, block) in BLOCK_DEFINITIONS.get().unwrap().iter().enumerate() {
            let mut variants = block.get_variants_len();
            let mut indexes = (0..variants)
                .into_iter()
                .enumerate()
                .map(|t| (BlockDefinitionIndex(i), t.0 as u32)).collect::<Vec<(BlockDefinitionIndex, BlockId)>>();
            self.block_index.append(&mut indexes);
        }

        self.visual_block_cache = vec![None; self.block_index.len()];
    }
}

/// A block in the context of the world
/// Stores a block definition along with its block uid
pub struct WorldBlock {
    definition: &'static BlockDefinition,
    block_id: BlockId
}

impl WorldBlock {
    #[inline]
    pub fn get_identifier(&self) -> &'static str {
        self.definition.identifier
    }

    #[inline]
    pub fn draw(&self) -> VisualBlock {
        self.definition.draw(self.block_id)
    }

    #[inline]
    pub fn get_loot(&self) -> Vec<LootTableEntry> {
        self.definition.get_loot(self.block_id)
    }
}