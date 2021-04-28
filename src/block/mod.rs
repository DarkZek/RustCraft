use crate::block::blocks::model::BlockModel;
use crate::block::blocks::BlockType;
use crate::services::asset_service::atlas::ATLAS_LOOKUPS;
use nalgebra::Vector3;
use std::collections::HashMap;

pub mod blocks;

//TODO: Update with real information about real in game blocks.

/// A struct to hold all of a blocks information. This is purely for internal purposes to store all blocks that exist in the game.
/// For physical blocks placed in the game use `PhysicalBlock`
#[derive(Clone, Debug)]
pub struct Block<'a> {
    pub block_type: &'a BlockType,
    pub block_state_index: usize,
}

impl Block<'_> {
    pub fn get_model(&self) -> BlockState {
        BlockState {
            model: self.block_type.get_model(ATLAS_LOOKUPS.get().unwrap()),
            model_transform_rot: Vector3::new(0.0, 0.0, 0.0),
            model_transform_pos: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

pub struct BlockState {
    pub model: BlockModel,
    pub model_transform_rot: Vector3<f32>,
    pub model_transform_pos: Vector3<f32>,
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

/// Stores a direction on a block. Useful for things like furnaces that have a different front face, face culling and hoppers that move items down or sideways.
#[derive(Copy, Clone, PartialEq)]
pub enum BlockDirection {
    Top = 0,
    Front = 1,
    Back = 2,
    Left = 3,
    Right = 4,
    Bottom = 5,
}
