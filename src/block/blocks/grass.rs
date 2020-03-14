use crate::block::Block;

pub const GRASS_BLOCK: Block = Block {
    id: 2,
    name: "Grass",
    raw_texture_names: ["glass"; 6],
    texture_ids: [0; 6],
    transparent: false,
};