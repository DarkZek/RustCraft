use crate::services::asset::atlas::atlas::TextureAtlas;
use crate::services::asset::atlas::resource_packs::ResourcePacks;
use crate::services::asset::atlas::{
    build_texture_atlas, load_resource_zips, AtlasLoadingStage, ResourcePackData,
};
use crate::services::asset::material::chunk::ChunkMaterial;
use crate::state::AppState;
use bevy::prelude::*;

pub mod atlas;
pub mod material;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AtlasLoadingStage::AwaitingIndex)
            .add_startup_system(create_asset_service)
            .add_system(load_resource_zips)
            .add_system_set(
                SystemSet::on_update(AppState::Loading).with_system(build_texture_atlas),
            );
    }
}

#[derive(Resource)]
pub struct AssetService {
    resource_packs: Handle<ResourcePacks>,
    pub texture_atlas: Option<TextureAtlas>,
    pack: Option<Handle<ResourcePackData>>,
    pub texture_atlas_material: Handle<ChunkMaterial>,
}

impl AssetService {
    pub fn new(server: Res<AssetServer>, materials: &mut Assets<ChunkMaterial>) -> AssetService {
        let texture_atlas_material = materials.add(ChunkMaterial {
            color: Color::GRAY,
            color_texture: None,
            alpha_mode: Default::default(),
        });

        AssetService {
            resource_packs: server.load("resources.json"),
            texture_atlas: None,
            pack: None,
            texture_atlas_material,
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
