use crate::services::asset::atlas::atlas::TextureAtlas;
use crate::services::asset::atlas::ResourcePackData;
use crate::{AssetServer, Commands, Handle, Res, ResMut, ResourcePacks, StandardMaterial};

pub mod atlas;

pub struct AssetService {
    resource_packs: Handle<ResourcePacks>,
    texture_atlas: TextureAtlas,
    pack: Option<Handle<ResourcePackData>>,
    pub texture_atlas_material: Option<Handle<StandardMaterial>>,
}

impl AssetService {
    pub fn new(server: Res<AssetServer>) -> AssetService {
        AssetService {
            resource_packs: server.load("resources.json"),
            texture_atlas: TextureAtlas::blank(),
            pack: None,
            texture_atlas_material: None,
        }
    }
}

pub fn create_asset_service(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(AssetService::new(assets));
}
