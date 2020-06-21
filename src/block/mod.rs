use crate::services::asset_service::atlas::TextureAtlasIndex;

pub mod blocks;

//TODO: Update with real information about real in game blocks.

/// A struct to hold all of a blocks information. This is purely for integral purposes to store all blocks that exist in the game.
/// For physical blocks placed in the game use `PhysicalBlock`
#[derive(Clone, Debug)]
pub struct Block {
    pub id: u32,
    pub name: &'static str,
    pub raw_texture_names: [&'static str; 6],
    pub texture_ids: [u32; 6],
    pub texture_atlas_lookups: [TextureAtlasIndex; 6],
    pub transparent: bool,
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
