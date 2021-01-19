use crate::services::asset_service::atlas::TextureAtlasIndex;
use crate::services::chunk_service::chunk::Color;
use nalgebra::Vector3;

pub mod blocks;

//TODO: Update with real information about real in game blocks.

/// A struct to hold all of a blocks information. This is purely for integral purposes to store all blocks that exist in the game.
/// For physical blocks placed in the game use `PhysicalBlock`
#[derive(Clone, Debug)]
pub struct Block {
    pub name: Option<String>,
    pub start_palette_id: u32,
    pub raw_texture_names: [String; 6],
    pub texture_atlas_lookups: [TextureAtlasIndex; 6],

    pub color: [u8; 3],
    pub light_intensity: u8,
    pub transparent: bool,
}

#[derive(Clone)]
pub enum ToolType {
    None,
    Pickaxe,
    Shovel,
    Sword,
    Hoe,
    Axe,
}

#[derive(Clone)]
pub struct BlockModel {}

impl BlockModel {
    //TODO: Implement
    pub fn get_textures(&self) -> Vec<&'static str> {
        vec![]
    }
    pub fn set_textures(&self, mappings: &mut Vec<([f32; 2], [f32; 2])>) {}
}

/// Stores a direction on a block. Useful for things like furnaces that have a different front face, face culling and hoppers that move items down or sideways.
#[derive(Copy, Clone)]
pub enum BlockDirection {
    Up = 0,
    Front = 1,
    Back = 2,
    Left = 3,
    Right = 4,
    Bottom = 5,
}
