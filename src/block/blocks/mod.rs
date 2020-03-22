use crate::block::Block;

mod dirt;
mod grass;
mod stone;
mod sand;
mod sandstone;

pub fn get_blocks() -> Vec<Block> {
    let mut blocks = Vec::new();

    blocks.push(stone::STONE_BLOCK);
    blocks.push(dirt::DIRT_BLOCK);
    blocks.push(grass::GRASS_BLOCK);
    blocks.push(sand::SAND_BLOCK);
    blocks.push(sandstone::SANDSTONE_BLOCK);

    blocks
}