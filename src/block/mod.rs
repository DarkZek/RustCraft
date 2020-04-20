use crate::services::asset_service::atlas::TextureAtlasIndex;

pub mod blocks;

#[derive(Clone)]
pub struct Block {
    pub id: u32,
    pub name: &'static str,
    pub raw_texture_names: [&'static str; 6],
    pub texture_ids: [u32; 6],
    pub texture_atlas_lookups: [TextureAtlasIndex; 6],
    pub transparent: bool,
}

#[derive(Copy, Clone)]
pub enum BlockDirection {
    Up = 0,
    Front = 1,
    Back = 2,
    Left = 3,
    Right = 4,
    Bottom = 5
}