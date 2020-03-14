use crate::block::Block;

pub mod dirt;
pub mod grass;
pub mod stone;

pub fn get_blocks() -> Vec<Block> {
    let mut blocks = Vec::new();

    blocks.push(stone::STONE_BLOCK);
    blocks.push(dirt::DIRT_BLOCK);
    blocks.push(grass::GRASS_BLOCK);

    blocks
}