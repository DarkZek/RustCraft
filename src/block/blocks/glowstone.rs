use crate::block::Block;

pub const GLOWSTONE_BLOCK: Block = Block {
    id: 6,
    name: "Glowstone",
    raw_texture_names: [
        "textures/block/glowstone",
        "textures/block/glowstone",
        "textures/block/glowstone",
        "textures/block/glowstone",
        "textures/block/glowstone",
        "textures/block/glowstone",
    ],
    texture_ids: [0; 6],
    texture_atlas_lookups: [([0.0, 0.0], [1.0, 1.0]); 6],
    transparent: false,
    light_intensity: 8,
    color: [0.5, 0.5, 0.0]
};
