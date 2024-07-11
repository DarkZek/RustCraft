use bevy::color::palettes::basic::{RED};
use bevy::pbr::ExtendedMaterial;
use crate::state::AppState;
use crate::systems::asset::atlas::resource_packs::{
    load_resource_zips, ResourcePackData, ResourcePacks,
};
use crate::systems::asset::atlas::{build_texture_atlas, AtlasLoadingStage};
use bevy::prelude::*;
use crate::systems::asset::material::chunk_extension::{ChunkMaterialExtension, ChunkMaterial};

pub mod atlas;
pub mod material;
pub mod parsing;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AtlasLoadingStage::AwaitingIndex)
            .add_systems(Startup, create_asset_service)
            .add_systems(Update, load_resource_zips)
            .add_systems(
                Update,
                build_texture_atlas.run_if(in_state(AppState::Loading)),
            );
    }
}

#[derive(Resource)]
pub struct AssetService {
    resource_packs: Handle<ResourcePacks>,
    pack: Option<Handle<ResourcePackData>>,
    pub opaque_texture_atlas_material: Handle<ChunkMaterial>,
    pub translucent_texture_atlas_material: Handle<ChunkMaterial>,
}

impl AssetService {
    pub fn new(server: Res<AssetServer>, materials: &mut Assets<ChunkMaterial>) -> AssetService {
        let opaque_texture_atlas_material = materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::from(RED),
                ..default()
            },
            extension: ChunkMaterialExtension {
                quantize_steps: 0
            },
        });
        let translucent_texture_atlas_material = materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::from(RED),
                ..default()
            },
            extension: ChunkMaterialExtension {
                quantize_steps: 0
            },
        });

        AssetService {
            resource_packs: server.load("resources.json"),
            pack: None,
            opaque_texture_atlas_material,
            translucent_texture_atlas_material,
        }
    }
}

pub fn create_asset_service(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
) {
    commands.insert_resource(AssetService::new(assets, &mut materials));
}
