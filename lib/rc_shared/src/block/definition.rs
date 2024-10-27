use std::sync::OnceLock;
use crate::block::BlockId;
use crate::block::blocks::stone::StoneBlock;
use crate::block::blocks::air::AirBlock;
use crate::block::blocks::{BlockImpl};
use crate::block::blocks::dirt::DirtBlock;
use crate::block::blocks::grass::GrassBlock;
use crate::block::blocks::lamp::LampBlock;
use crate::block::blocks::leaves::LeavesBlock;
use crate::block::blocks::long_grass::LongGrassBlock;
use crate::block::blocks::pipe::PipeBlock;
use crate::block::blocks::plaster::PlasterBlock;
use crate::block::blocks::sand::SandBlock;
use crate::block::blocks::water::WaterBlock;
use crate::block::blocks::wood_log::WoodLogBlock;
use crate::block::types::{VisualBlock, LootTableEntry};

pub(crate) static BLOCK_DEFINITIONS: OnceLock<Vec<BlockDefinition>> = OnceLock::new();

pub(crate) fn set_blocks() {
    BLOCK_DEFINITIONS.set(vec![
        BlockDefinition::from_block_impl::<AirBlock>(),
        BlockDefinition::from_block_impl::<DirtBlock>(),
        BlockDefinition::from_block_impl::<GrassBlock>(),
        BlockDefinition::from_block_impl::<LongGrassBlock>(),
        BlockDefinition::from_block_impl::<WoodLogBlock>(),
        BlockDefinition::from_block_impl::<LeavesBlock>(),
        BlockDefinition::from_block_impl::<StoneBlock>(),
        BlockDefinition::from_block_impl::<LampBlock>(),
        BlockDefinition::from_block_impl::<SandBlock>(),
        BlockDefinition::from_block_impl::<PipeBlock>(),
        BlockDefinition::from_block_impl::<PlasterBlock>(),
        BlockDefinition::from_block_impl::<WaterBlock>(),
    ]).unwrap();
}

#[derive(Debug)]
pub struct BlockDefinition {
    pub identifier: &'static str,
    get_variants: fn() -> Vec<VisualBlock>,
    draw: fn(BlockId) -> VisualBlock,
    get_loot: fn(BlockId) -> Vec<LootTableEntry>,
    on_destroy: fn(BlockId),
}

impl BlockDefinition {
    pub fn from_block_impl<T: BlockImpl>() -> Self {
        Self {
            identifier: T::IDENTIFIER,
            get_variants: T::get_variants,
            on_destroy: |uid| T::parse_block_state(uid).on_destroy(),
            draw: |uid| T::parse_block_state(uid).draw(),
            get_loot: |uid| T::parse_block_state(uid).get_loot()
        }
    }

    pub fn get_variants_len(&self) -> usize {
        (self.get_variants)().len()
    }

    pub fn draw(&self, uid: BlockId) -> VisualBlock {
        (self.draw)(uid)
    }

    pub fn get_loot(&self, uid: BlockId) -> Vec<LootTableEntry> {
        (self.get_loot)(uid)
    }
}