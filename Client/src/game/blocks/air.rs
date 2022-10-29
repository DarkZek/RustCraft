use crate::game::blocks::Block;
use crate::game::mesh::face::Face;
use crate::services::asset::atlas::atlas::ATLAS_WIDTH;
use crate::services::asset::atlas::index::TextureAtlasIndex;

pub fn generate_air_block() -> Block {
    Block {
        name: "Air".to_string(),
        identifier: "mcv3::air".to_string(),
        translucent: true,
        full: false,
        draw_betweens: false,
        faces: Vec::new(),
    }
}
