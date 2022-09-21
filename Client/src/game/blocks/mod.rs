pub mod dirt;

use crate::game::blocks::dirt::DIRT_BLOCK_GENERATOR;
use crate::game::mesh::draw_kit::DrawKit;
use crate::game::viewable_direction::{ViewableDirection, ViewableDirectionBitMap};
use nalgebra::Vector3;

pub struct BlockEntry {
    id: usize,
    block: &'static dyn BlockGenerator<'static>,
}

pub trait BlockGenerator<'a>: Sync + Send {
    /// Gets the amount of index's this block should cover
    fn indexes(&self) -> usize;

    /// Creates a block with index data
    fn create(&self, index: usize) -> Box<dyn Block>;
}

pub trait Block: Sync + Send {
    /// Gets the blocks name
    fn name(&self) -> &'static str;

    /// Gets the blocks identifier
    fn identifier(&self) -> &'static str;

    /// Calculates if the block is translucent
    fn is_translucent(&self) -> bool;

    /// Calculates if the block is full (takes up entire 1m^3 space)
    fn is_full(&self) -> bool;

    /// Calculates if sides of block should be rendered if its transparent with the same block type on the side
    fn draw_betweens(&self) -> bool;

    /// Draws the blocks visuals into the buffers
    fn draw(&self, position: Vector3<f32>, visibility: ViewableDirection, draw: DrawKit);
}

pub struct BlockStates {
    pub states: Vec<BlockEntry>,
}
impl BlockStates {
    pub fn get_block(&self, id: usize) -> Option<Box<dyn Block>> {
        self.states
            .get(id as usize)
            .map(|block| block.block.create(id - block.id))
    }

    pub fn new() -> BlockStates {
        let mut states = Vec::new();

        let mut generators: Vec<&'static dyn BlockGenerator> = Vec::new();
        generators.push(&DIRT_BLOCK_GENERATOR);
        generators.push(&DIRT_BLOCK_GENERATOR);

        let mut index = 0;

        for generator in generators {
            let generator_index = index;

            for _ in 0..generator.indexes() {
                states.push(BlockEntry {
                    id: generator_index,
                    block: generator,
                });

                index += 1;
            }
        }

        BlockStates { states }
    }
}
