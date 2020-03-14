use crate::block::Block;

pub const DIRT_BLOCK: Block = Block {
    id: 3,
    name: "Dirt",
    raw_texture_names: ["dirt"; 6],
    texture_ids: [0; 6],
    transparent: false,
};