use crate::systems::asset::atlas::atlas::{TextureAtlas, TEXTURE_ATLAS};
use crate::systems::asset::atlas::resource_packs::{ResourcePackData, ResourcePacks};
use crate::systems::asset::material::chunk::ChunkMaterial;
use crate::systems::asset::AssetService;
use crate::systems::ui::loading::LoadingUIData;
use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::reflect::TypeUuid;
use fnv::FnvBuildHasher;
use image::DynamicImage;
use std::collections::HashMap;
use std::ffi::OsString;

pub mod atlas;
pub mod resource_packs;

#[derive(Debug, PartialEq, Eq, Resource)]
pub enum AtlasLoadingStage {
    AwaitingIndex,
    AwaitingPack,
    Done,
}

pub fn build_texture_atlas(
    packs: Res<Assets<ResourcePacks>>,
    mut data: ResMut<Assets<ResourcePackData>>,
    mut service: ResMut<AssetService>,
    mut stage: ResMut<AtlasLoadingStage>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    mut loading: ResMut<LoadingUIData>,
) {
    if *stage != AtlasLoadingStage::AwaitingPack
        || data.len() == 0
        || *stage == AtlasLoadingStage::Done
    {
        return;
    }

    // Fetch the resources required to build the texture atlas
    let pack = packs.get(&service.resource_packs).unwrap().get_default();
    let textures = data.get_mut(service.pack.as_ref().unwrap());

    let textures = match textures {
        None => return,
        Some(val) => val,
    };

    // Build the texture atlas
    let atlas = TextureAtlas::new(pack, &mut textures.images, &mut images);

    info!("Generated texture atlas");
    TEXTURE_ATLAS.set(atlas);

    // Create a new material
    let _ = materials.insert(
        &service.opaque_texture_atlas_material,
        ChunkMaterial {
            color: Color::WHITE,
            color_texture: Some(TEXTURE_ATLAS.get().get_image().clone()),
            alpha_mode: AlphaMode::Mask(0.2),
        },
    );

    let _ = materials.insert(
        &service.translucent_texture_atlas_material,
        ChunkMaterial {
            color: Color::WHITE,
            color_texture: Some(TEXTURE_ATLAS.get().get_image().clone()),
            alpha_mode: AlphaMode::Mask(0.2), // Culling happens in custom shader
        },
    );

    *stage = AtlasLoadingStage::Done;
    loading.texture_atlas = true;
}
