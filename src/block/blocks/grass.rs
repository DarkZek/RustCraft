use crate::block::Block;

pub const GRASS_BLOCK: Block = Block {
    id: 2,
    name: "Grass",
    raw_texture_names: [
        "textures/block/grass_block_top",
        "textures/block/grass_block_side",
        "textures/block/grass_block_side",
        "textures/block/grass_block_side",
        "textures/block/grass_block_side",
        "textures/block/dirt",
    ],
    texture_ids: [0; 6],
    texture_atlas_lookups: [([0.0, 0.0], [1.0, 1.0]); 6],
    transparent: false,
    light_intensity: 0,
    color: [0, 0, 0],
};
