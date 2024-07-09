use bevy::color::palettes::basic::GRAY;
use crate::state::AppState;

use crate::systems::asset::atlas::resource_packs::{
    load_resource_zips, ResourcePackData, ResourcePacks,
};
use crate::systems::asset::atlas::{build_texture_atlas, AtlasLoadingStage};
use crate::systems::asset::material::chunk::ChunkMaterial;
use bevy::prelude::*;

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
        let opaque_texture_atlas_material = materials.add(ChunkMaterial {
            color: LinearRgba::from(GRAY),
            color_texture: None,
            alpha_mode: Default::default(),
        });
        let translucent_texture_atlas_material = materials.add(ChunkMaterial {
            color: LinearRgba::from(GRAY),
            color_texture: None,
            alpha_mode: Default::default(),
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
