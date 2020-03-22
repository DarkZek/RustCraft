use crate::block::Block;

pub const GRASS_BLOCK: Block = Block {
    id: 2,
    name: "Grass",
    raw_texture_names: ["grass_top", "grass_side", "grass_side", "grass_side", "grass_side", "dirt"],
    texture_ids: [0; 6],
    texture_atlas_lookups: [([0.0, 0.0], [1.0, 1.0]); 6],
    transparent: false,
};