use crate::block::Block;

mod dirt;
mod grass;
mod sand;
mod sandstone;
mod stone;

//TODO: Remove this and load blocks dynamically

/// To be removed
pub fn get_blocks() -> Vec<Block> {
    let mut blocks = Vec::new();

    blocks.push(stone::STONE_BLOCK);
    blocks.push(dirt::DIRT_BLOCK);
    blocks.push(grass::GRASS_BLOCK);
    blocks.push(sand::SAND_BLOCK);
    blocks.push(sandstone::SANDSTONE_BLOCK);

    blocks
}
