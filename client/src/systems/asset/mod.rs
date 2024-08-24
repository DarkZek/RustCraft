use bevy::color::palettes::basic::{RED};
use bevy::pbr::ExtendedMaterial;
use crate::state::AppState;
use crate::systems::asset::atlas::resource_packs::{
    load_resource_zips, ResourcePackData, ResourcePacks,
};
use crate::systems::asset::atlas::{build_texture_atlas, AtlasLoadingStage};
use bevy::prelude::*;
use crate::systems::asset::material::chunk_extension::{ChunkMaterialExtension, ChunkMaterial};
use crate::systems::asset::material::time::update_time;
use crate::systems::asset::material::translucent_chunk_extension::{ChunkMaterialUniform, TranslucentChunkMaterial, TranslucentChunkMaterialExtension};

pub mod atlas;
pub mod material;
pub mod parsing;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AtlasLoadingStage::AwaitingIndex)
            .add_systems(Startup, create_asset_service)
            .add_systems(Update, load_resource_zips)
            .add_systems(Update, update_time)
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
    pub translucent_texture_atlas_material: Handle<TranslucentChunkMaterial>,
}

impl AssetService {
    pub fn new(server: Res<AssetServer>, materials: &mut Assets<ChunkMaterial>, translucent_materials: &mut Assets<TranslucentChunkMaterial>) -> AssetService {
        let opaque_texture_atlas_material = materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::from(RED),
                ..default()
            },
            extension: ChunkMaterialExtension { uniform: ChunkMaterialUniform { ambient_strength: 0.0, ..default() } },
        });
        let translucent_texture_atlas_material = translucent_materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::from(RED),
                ..default()
            },
            extension: TranslucentChunkMaterialExtension { uniform: ChunkMaterialUniform { time: 0.0, ambient_strength: 0.0, ..default() } },
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
    mut translucent_materials: ResMut<Assets<TranslucentChunkMaterial>>,
) {
    commands.insert_resource(AssetService::new(assets, &mut materials, &mut translucent_materials));
}
