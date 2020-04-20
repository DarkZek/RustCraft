//
// Handles loading assets, the texture atlas and resource packs
//

use std::collections::HashMap;
use crate::services::settings_service::SettingsService;
use image::DynamicImage;
use wgpu::{Texture, Sampler};
use crate::services::asset_service::atlas::TextureAtlasIndex;
use crate::services::{ServicesContext};
use crate::services::asset_service::blocks_array::generate_blocks_array;

pub mod depth_map;
pub mod binding;
pub mod atlas;
pub mod packs;
pub mod blocks_array;

pub struct AssetService {
    resource_packs: Vec<String>,
    selected_pack: Option<ResourcePack>,
    pub texture_atlas: Option<Texture>,
    pub texture_atlas_index: Option<HashMap<String, TextureAtlasIndex>>,
    pub texture_sampler: Option<Sampler>,
    pub blocks_texture: Option<Texture>,
    pub blocks_sampler: Option<Sampler>
}

pub struct ResourcePack {
    name: String,
    author: String,
    version: String,
    textures: HashMap<String, DynamicImage>
}

impl AssetService {
    pub fn new(settings: &SettingsService, context: &mut ServicesContext) -> AssetService {
        let resource_packs = AssetService::get_resource_packs((settings.path.as_str().to_owned() + "resources/").as_ref());

        log!("Resource Packs: {:?}", resource_packs);

        // For now, select the first one in the list. In the future we will grab the selected resource pack from the settings
        let selected_pack = resource_packs.get(0);
        let mut selected_pack = if let Some(pack) = selected_pack {
            Some(AssetService::load_resource_pack(&format!("{}resources/{}", settings.path, pack)))
        } else {
            None
        };

        let (texture_atlas, texture_atlas_index, texture_sampler) =
            AssetService::generate_texture_atlas(selected_pack.as_mut().unwrap(), context.device, context.queue, settings);

        let (blocks_texture, blocks_sampler) = generate_blocks_array(context);

        AssetService {
            resource_packs,
            selected_pack,
            texture_atlas: Some(texture_atlas),
            texture_atlas_index: Some(texture_atlas_index),
            texture_sampler: Some(texture_sampler),
            blocks_texture: Some(blocks_texture),
            blocks_sampler: Some(blocks_sampler)
        }
    }
}