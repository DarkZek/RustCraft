use crate::block::Block;

pub const STONE_BLOCK: Block = Block {
    id: 1,
    name: "Stone",
    raw_texture_names: ["stone"; 6],
    texture_ids: [0; 6],
    transparent: false,
};