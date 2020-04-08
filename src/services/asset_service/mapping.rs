use crate::block::Block;
use crate::services::asset_service::atlas::{TextureAtlasIndex};
use std::collections::HashMap;

/// Updates block list with UV coordinates of it's textures on the texture atlas
pub fn load_textures(blocks: &mut Vec<Block>, maps: &HashMap<String, TextureAtlasIndex>) {
    //TODO: This
    // Apply atlas lookups
    // for block in blocks {
    //     for (i, atlas_id) in block.texture_ids.iter().enumerate() {
    //         let index = atlas.get(atlas_id.clone() as usize).unwrap();
    //         block.texture_atlas_lookups[i] = index.clone();
    //     }
    // }
}

fn index_of(array: &Vec<&str>, search: &str) -> Option<u32> {
    for (i, value) in array.iter().enumerate() {
        if value == &search {
            return Some(i as u32);
        }
    }

    None
}