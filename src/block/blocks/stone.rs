use crate::block::Block;

pub const STONE_BLOCK: Block = Block {
    id: 1,
    name: "Stone",
    raw_texture_names: ["textures/block/stone"; 6],
    texture_ids: [0; 6],
    texture_atlas_lookups: [([0.0, 0.0], [1.0, 1.0]); 6],
    transparent: false,
    light_intensity: 0,
    color: [0.0, 0.0, 0.0]
};
