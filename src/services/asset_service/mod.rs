//
// Handles loading assets, the texture atlas and resource packs
//

use crate::services::asset_service::atlas::TextureAtlasIndex;
use crate::services::settings_service::SettingsService;
use crate::services::ServicesContext;
use image::DynamicImage;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::time::SystemTime;
use wgpu::{BindGroup, BindGroupLayout, Sampler, Texture};

pub mod atlas;
pub mod binding;
pub mod depth_map;
pub mod packs;

#[allow(dead_code)]
pub struct AssetService {
    resource_packs: Vec<String>,
    selected_pack: Option<ResourcePack>,
    pub atlas_image: Option<DynamicImage>,
    pub atlas: Option<Texture>,
    pub atlas_index: Option<HashMap<String, TextureAtlasIndex>>,
    pub atlas_sampler: Option<Sampler>,
    pub atlas_bind_group_layout: Option<BindGroupLayout>,
    pub atlas_bind_group: Option<BindGroup>,
}

#[allow(dead_code)]
pub struct ResourcePack {
    name: String,
    author: String,
    version: String,
    textures: HashMap<String, DynamicImage>,
    modified: SystemTime,
}

impl AssetService {
    pub fn new(settings: &SettingsService, context: &mut ServicesContext) -> AssetService {
        let resource_packs = AssetService::get_resource_packs(
            (settings.path.as_str().to_owned() + "resources/").as_ref(),
        );

        if resource_packs.len() == 0 {
            panic!("No resource packs found!");
        }

        log!("Resource Packs: {:?}", resource_packs);

        // For now, select the first one in the list. In the future we will grab the selected resource pack from the settings
        let mut selected_pack = AssetService::load_resource_pack(&format!(
            "{}resources/{}",
            settings.path,
            resource_packs.get(0).unwrap()
        ));

        let (atlas_image, atlas, atlas_index, atlas_sampler) = AssetService::generate_texture_atlas(
            &mut selected_pack,
            resource_packs.get(0).unwrap(),
            context.device.as_ref(),
            context.queue.lock().unwrap().deref_mut(),
            settings,
        );

        let (atlas_bind_group_layout, atlas_bind_group) =
            AssetService::generate_atlas_bindings(&mut context.device, &atlas, &atlas_sampler);

        AssetService {
            resource_packs,
            selected_pack: Some(selected_pack),
            atlas_image: Some(atlas_image),
            atlas: Some(atlas),
            atlas_index: Some(atlas_index),
            atlas_sampler: Some(atlas_sampler),
            atlas_bind_group_layout: Some(atlas_bind_group_layout),
            atlas_bind_group: Some(atlas_bind_group),
        }
    }
}

impl Default for AssetService {
    fn default() -> Self {
        unimplemented!()
    }
}
