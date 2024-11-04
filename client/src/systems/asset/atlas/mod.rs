use crate::systems::asset::atlas::resource_packs::{ResourcePackData, ResourcePacks};
use crate::systems::asset::AssetService;
use crate::systems::ui::loading::LoadingUIData;

use bevy::prelude::*;
use bevy::color::palettes::basic::WHITE;
use bevy::pbr::ExtendedMaterial;
use rc_particle::ParticleResource;
use rc_shared::atlas::TEXTURE_ATLAS;
use crate::systems::asset::atlas::atlas::new_atlas;
use crate::systems::asset::material::translucent_chunk_extension::ChunkMaterialUniform;
use crate::systems::asset::material::chunk_extension::{ChunkMaterialExtension, ChunkMaterial};
use crate::systems::asset::material::translucent_chunk_extension::{TranslucentChunkMaterial, TranslucentChunkMaterialExtension};

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
    service: ResMut<AssetService>,
    mut stage: ResMut<AtlasLoadingStage>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    mut translucent_materials: ResMut<Assets<TranslucentChunkMaterial>>,
    mut loading: ResMut<LoadingUIData>
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
    let atlas = new_atlas(pack, &mut textures.images, &mut images);

    info!("Generated texture atlas");
    TEXTURE_ATLAS.set(atlas);

    // Create a new material
    let _ = materials.insert(
        &service.opaque_texture_atlas_material,
        ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::from(WHITE),
                base_color_texture: Some(TEXTURE_ATLAS.get().get_image().clone()),
                alpha_mode: AlphaMode::Opaque,
                perceptual_roughness: 0.0,
                reflectance: 0.0,
                ..default()
            },
            extension: ChunkMaterialExtension { uniform: ChunkMaterialUniform { ambient_strength: 0.04, ..default() } },
        },
    );

    let _ = translucent_materials.insert(
        &service.translucent_texture_atlas_material,
        ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::from(WHITE),
                base_color_texture: Some(TEXTURE_ATLAS.get().get_image().clone()),
                alpha_mode: AlphaMode::Mask(0.5),
                perceptual_roughness: 0.0,
                reflectance: 0.0,
                ..default()
            },
            extension: TranslucentChunkMaterialExtension { uniform: ChunkMaterialUniform { time: 0.0, ambient_strength: 0.04, ..default() } },
        },
    );

    *stage = AtlasLoadingStage::Done;
    loading.texture_atlas = true;
}
