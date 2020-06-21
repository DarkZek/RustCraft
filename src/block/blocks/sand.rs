use crate::block::Block;

pub const SAND_BLOCK: Block = Block {
    id: 4,
    name: "Sand",
    raw_texture_names: ["textures/block/sand"; 6],
    texture_ids: [0; 6],
    texture_atlas_lookups: [([0.0, 0.0], [1.0, 1.0]); 6],
    transparent: false,
};
