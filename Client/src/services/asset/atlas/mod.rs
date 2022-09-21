use crate::services::asset::atlas::atlas::TextureAtlas;
use crate::services::asset::atlas::resource_packs::{ResourcePack, ResourcePacks};
use crate::services::asset::AssetService;
use crate::KeyCode::At;
use crate::{
    default, error, info, warn, AlphaMode, AssetServer, Assets, Changed, ChunkData, Color,
    Commands, DetectChanges, Entity, Handle, Image, PbrBundle, Query, Res, ResMut,
    StandardMaterial, With,
};
use bevy::asset::{
    create_platform_default_asset_io, AssetIoError, AssetLoader, BoxedFuture, LoadContext,
    LoadedAsset,
};
use bevy::reflect::TypeUuid;
use fnv::FnvBuildHasher;
use image::DynamicImage;
use serde_json::Value;
use std::collections::HashMap;
use std::ffi::OsString;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::task::Poll;
use std::thread::Thread;
use std::{mem, thread};

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
    mut chunks: Query<Entity, (With<ChunkData>, With<Handle<StandardMaterial>>)>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    if *stage != AtlasLoadingStage::AwaitingPack || data.len() == 0 {
        return;
    }

    // Fetch the resources required to build the texture atlas
    let pack = packs.get(&service.resource_packs).unwrap().get_default();
    let textures = data.get_mut(service.pack.as_ref().unwrap());

    if textures.is_none() {
        return;
    }

    // Build the texture atlas
    let atlas = TextureAtlas::new(pack, &mut textures.unwrap().images, &mut images);

    info!("Generated texture atlas");
    service.texture_atlas = atlas;

    // Create a new material
    let material = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
        base_color_texture: Some(images.get_handle(service.texture_atlas.get_image())),
        alpha_mode: AlphaMode::Opaque,
        unlit: true,
        ..default()
    });

    for chunk in chunks.iter() {
        commands.entity(chunk).insert(material.clone());
    }

    *stage = AtlasLoadingStage::Done;
    service.texture_atlas_material = Some(material);
}
