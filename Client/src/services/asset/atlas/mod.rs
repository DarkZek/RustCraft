use crate::services::asset::atlas::atlas::TextureAtlas;
use crate::services::asset::atlas::resource_packs::ResourcePacks;
use crate::services::asset::material::chunk::ChunkMaterial;
use crate::services::asset::AssetService;

use crate::{
    error, info, AlphaMode, AssetServer, Assets, ChunkData, Color, Commands, Entity, Handle, Image,
    Query, Res, ResMut, StandardMaterial, With,
};
use bevy::reflect::TypeUuid;
use fnv::FnvBuildHasher;
use image::{DynamicImage, GenericImage};

use bevy_inspector_egui::egui::TextBuffer;
use std::collections::HashMap;
use std::ffi::OsString;

pub mod atlas;
pub mod index;
pub mod resource_packs;

#[derive(Debug, PartialEq, Eq)]
pub enum AtlasLoadingStage {
    AwaitingIndex,
    AwaitingPack,
    Done,
}

/// The images that make up a resource pack
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "7b14806a-672b-423b-8d16-4f18afefa463"]
pub struct ResourcePackData {
    images: HashMap<String, DynamicImage, FnvBuildHasher>,
}

impl ResourcePackData {
    pub fn new(images: HashMap<String, DynamicImage, FnvBuildHasher>) -> ResourcePackData {
        ResourcePackData { images }
    }
}

pub fn load_resource_zips(
    packs: Res<Assets<ResourcePacks>>,
    mut service: ResMut<AssetService>,
    server: Res<AssetServer>,
    mut stage: ResMut<AtlasLoadingStage>,
) {
    // Only load zips on change to resource packs
    if *stage != AtlasLoadingStage::AwaitingIndex || packs.len() == 0 {
        return;
    }
    if !packs.is_changed() {
        return;
    }

    let packs = packs.get(&service.resource_packs).unwrap();

    let pack = packs.get_default();

    if !pack.path.extension().unwrap().eq(&OsString::from("pack")) {
        error!(
            "Resource pack {:?} does not end with .pack, aborting load.",
            pack.path
        );
        return;
    }

    service.pack = Some(server.load(pack.path.clone()));

    *stage = AtlasLoadingStage::AwaitingPack;
}

pub fn build_texture_atlas(
    packs: Res<Assets<ResourcePacks>>,
    mut data: ResMut<Assets<ResourcePackData>>,
    mut service: ResMut<AssetService>,
    mut stage: ResMut<AtlasLoadingStage>,
    chunks: Query<Entity, (With<ChunkData>, With<Handle<StandardMaterial>>)>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    mut commands: Commands,
) {
    if *stage != AtlasLoadingStage::AwaitingPack || data.len() == 0 {
        return;
    }

    // Fetch the resources required to build the texture atlas
    let pack = packs.get(&service.resource_packs).unwrap().get_default();
    let textures = data.get_mut(service.pack.as_ref().unwrap());

    let textures = match textures {
        None => return,
        Some(val) => val,
    };

    // Add invalid texture
    if !textures.images.contains_key("game/invalid") {
        let mut img = DynamicImage::new_rgb8(2, 2);
        img.put_pixel(0, 0, image::Rgba([255, 0, 255, 255]));
        img.put_pixel(1, 1, image::Rgba([255, 0, 255, 255]));
        textures.images.insert("game/invalid".to_string(), img);
    }

    // Build the texture atlas
    let atlas = TextureAtlas::new(pack, &mut textures.images, &mut images);

    info!("Generated texture atlas");
    service.texture_atlas = Some(atlas);

    // Create a new material
    let material = materials.set(
        service.texture_atlas_material.id,
        ChunkMaterial {
            color: Color::WHITE,
            color_texture: Some(
                images.get_handle(service.texture_atlas.as_ref().unwrap().get_image()),
            ),
            alpha_mode: AlphaMode::Opaque,
        },
    );

    for chunk in chunks.iter() {
        commands.entity(chunk).insert(material.clone());
    }

    *stage = AtlasLoadingStage::Done;
    service.texture_atlas_material = material;
}
