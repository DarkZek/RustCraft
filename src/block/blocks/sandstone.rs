use crate::block::Block;

pub const SANDSTONE_BLOCK: Block = Block {
    id: 5,
    name: "Sandstone",
    raw_texture_names: ["sandstone_top", "sandstone_normal", "sandstone_normal", "sandstone_normal", "sandstone_normal", "sandstone_bottom"],
    texture_ids: [0; 6],
    texture_atlas_lookups: [([0.0, 0.0], [1.0, 1.0]); 6],
    transparent: false,
};