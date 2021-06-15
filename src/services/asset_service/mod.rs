//
// Handles loading assets, the texture atlas and resource packs
//

use crate::services::asset_service::index::TextureAtlasIndex;
use crate::services::settings_service::SettingsService;
use crate::services::ServicesContext;
use image::DynamicImage;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::time::SystemTime;
use wgpu::{BindGroup, BindGroupLayout, Sampler, Texture};
use std::fs;
use std::io::Cursor;
use std::error::Error;
use std::path::PathBuf;
use native_dialog::{MessageDialog, MessageType};

pub mod atlas;
pub mod binding;
pub mod depth_map;
pub mod index;
pub mod packs;

static DEFAULT_RESOURCE_PACK: &str = "Faithful.zip";

#[allow(dead_code)]
pub struct AssetService {
    resource_packs: Vec<String>,
    selected_pack: Option<ResourcePack>,
    pub atlas_image: Option<DynamicImage>,
    pub atlas: Option<Texture>,
    // TODO: Change from using string based lookup system to using hashed id's internally, and also add direct access via vec and make hashmap simply give index to vec
    // also cache popular block models for faster chunk gen
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

        let mut path = settings.path.clone();
        path.push("resources");

        // Try creating resources directory
        fs::create_dir_all(path.as_path());

        let mut resource_packs = AssetService::get_resource_packs(path.clone());

        if resource_packs.len() == 0 {

            log_error!("No resource packs found");

            // Ask the user if they would like to download the default resources
            let result = MessageDialog::new()
                .set_type(MessageType::Info)
                .set_title("No resource packs found")
                .set_text(&format!("No resource packs installed. Would you like to automatically download {}?", DEFAULT_RESOURCE_PACK))
                .show_confirm();

            // Ensure the message got through and respond to its input
            match result {
                Ok(download_default) => {
                    if !download_default {
                        std::process::exit(0);
                    }
                }
                Err(err) => {
                    log_error!("Failed to display popup for resource packs: {:?}", err);
                    std::process::exit(0);
                }
            }

            // Download the default resources
            if let Result::Err(err) = AssetService::download_default_resources(path.clone()) {
                log_error!("Failed to download Rosources.zip: {:?}", err);
                MessageDialog::new()
                    .set_type(MessageType::Error)
                    .set_title(&*format!("Failed to download {}", DEFAULT_RESOURCE_PACK))
                    .set_text(&format!("Failed to download {}, check your network connection.\n{:?}", DEFAULT_RESOURCE_PACK, err))
                    .show_alert();
                std::process::exit(0);
            }

            log!("Downloaded {}", DEFAULT_RESOURCE_PACK);

            // Find resource packs again
            resource_packs = AssetService::get_resource_packs(path.clone());
        }

        log!("Resource Packs: {:?}", resource_packs);

        path.push(resource_packs.get(0).unwrap());

        // For now, select the first one in the list. In the future we will grab the selected resource pack from the settings
        let mut selected_pack = match AssetService::load_resource_pack(path.clone()) {
            Ok(val) => val,
            Err(err) => {
                log_error!("Error loading zip {:?}: {}", &path, err);
                std::process::exit(0);
            }
        };

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

    pub fn download_default_resources(mut pack_path: PathBuf) -> Result<(), Box<dyn Error>> {
        let result = reqwest::blocking::get("https://github.com/FaithfulTeam/Faithful/blob/releases/1.15.zip?raw=true").unwrap();

        pack_path.push(DEFAULT_RESOURCE_PACK);

        // Save
        let mut file = std::fs::File::create(pack_path)?;
        let mut content =  Cursor::new(result.bytes()?);
        std::io::copy(&mut content, &mut file)?;
        Ok(())
    }
}

impl Default for AssetService {
    fn default() -> Self {
        unimplemented!()
    }
}
