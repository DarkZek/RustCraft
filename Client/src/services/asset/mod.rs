use crate::services::asset::atlas::atlas::TextureAtlas;
use crate::services::asset::atlas::ResourcePackData;
use crate::services::asset::material::chunk::ChunkMaterial;
use crate::{AssetServer, Commands, Handle, Res, ResMut, ResourcePacks};
use bevy::prelude::*;

pub mod atlas;
pub mod material;

pub struct AssetService {
    resource_packs: Handle<ResourcePacks>,
    texture_atlas: TextureAtlas,
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
            texture_atlas: TextureAtlas::blank(),
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
