use crate::block::Block;

pub const DIRT_BLOCK: Block = Block {
    id: 3,
    name: "Dirt",
    raw_texture_names: ["textures/block/dirt"; 6],
    texture_ids: [0; 6],
    texture_atlas_lookups: [([0.0, 0.0], [1.0, 1.0]); 6],
    transparent: false,
};