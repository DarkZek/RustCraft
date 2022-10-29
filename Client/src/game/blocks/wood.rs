use crate::game::blocks::Block;
use crate::game::mesh::face::Face;
use crate::services::asset::atlas::atlas::ATLAS_WIDTH;
use crate::services::asset::atlas::index::TextureAtlasIndex;

pub fn generate_wood_block() -> Block {
    // Temp until atlas lookups are ported from mcv3v1
    let i = 16.0 / ATLAS_WIDTH as f32;
    let dirt = TextureAtlasIndex {
        u_min: i,
        u_max: i * 2.0,
        v_min: 0.0,
        v_max: i,
    };

    Block {
        name: "Tree Wood".to_string(),
        identifier: "mcv3::tree_wood".to_string(),
        translucent: false,
        full: true,
        draw_betweens: false,
        faces: Vec::from(Face::full_block(dirt)),
    }
}
