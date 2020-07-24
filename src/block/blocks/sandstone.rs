use crate::block::Block;

pub const SANDSTONE_BLOCK: Block = Block {
    id: 5,
    name: "Sandstone",
    raw_texture_names: [
        "textures/block/sandstone_top",
        "textures/block/sandstone",
        "textures/block/sandstone",
        "textures/block/sandstone",
        "textures/block/sandstone",
        "textures/block/sandstone_bottom",
    ],
    texture_ids: [0; 6],
    texture_atlas_lookups: [([0.0, 0.0], [1.0, 1.0]); 6],
    transparent: false,
    light_intensity: 14,
    color: [51, 51, 255],
};
