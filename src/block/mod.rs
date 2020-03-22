use crate::render::texture::atlas::TextureAtlasIndex;

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