use crate::game::blocks::states::BlockStates;
use crate::game::blocks::Block;
use crate::services::asset::atlas::index::Rotate;
use crate::services::chunk::data::{ChunkData, RawChunkData};
use bevy::prelude::warn;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ViewableDirection(pub u8);

#[derive(Clone, Copy, PartialEq, Debug, Deserialize, Serialize)]
pub enum ViewableDirectionBitMap {
    Top = 0b00000001,
    Bottom = 0b00000010,
    Left = 0b00000100,
    Right = 0b00001000,
    Front = 0b00010000,
    Back = 0b00100000,
}

impl ViewableDirectionBitMap {
    pub fn from_code(code: u8) -> Option<ViewableDirectionBitMap> {
        match code {
            0b00000001 => Some(ViewableDirectionBitMap::Top),
            0b00000010 => Some(ViewableDirectionBitMap::Bottom),
            0b00000100 => Some(ViewableDirectionBitMap::Left),
            0b00001000 => Some(ViewableDirectionBitMap::Right),
            0b00010000 => Some(ViewableDirectionBitMap::Front),
            0b00100000 => Some(ViewableDirectionBitMap::Back),
            _ => None,
        }
    }

    pub fn from(direction: &Vector3<i32>) -> ViewableDirectionBitMap {
        if direction.z > 0 {
            ViewableDirectionBitMap::Back
        } else if direction.z < 0 {
            ViewableDirectionBitMap::Front
        } else if direction.y > 0 {
            ViewableDirectionBitMap::Top
        } else if direction.y < 0 {
            ViewableDirectionBitMap::Bottom
        } else if direction.x < 0 {
            ViewableDirectionBitMap::Left
        } else if direction.x > 0 {
            ViewableDirectionBitMap::Right
        } else {
            ViewableDirectionBitMap::Top
        }
    }

    pub fn invert(&self) -> ViewableDirectionBitMap {
        match self {
            ViewableDirectionBitMap::Top => ViewableDirectionBitMap::Bottom,
            ViewableDirectionBitMap::Bottom => ViewableDirectionBitMap::Top,
            ViewableDirectionBitMap::Left => ViewableDirectionBitMap::Right,
            ViewableDirectionBitMap::Right => ViewableDirectionBitMap::Left,
            ViewableDirectionBitMap::Front => ViewableDirectionBitMap::Back,
            ViewableDirectionBitMap::Back => ViewableDirectionBitMap::Front,
        }
    }

    pub fn rotate(&self, deg: Rotate) -> ViewableDirectionBitMap {
        match deg {
            // Rotate::Deg270 => match self {
            //     ViewableDirectionBitMap::Top => ViewableDirectionBitMap::Top,
            //     ViewableDirectionBitMap::Bottom => ViewableDirectionBitMap::Top,
            //     ViewableDirectionBitMap::Left => ViewableDirectionBitMap::Front,
            //     ViewableDirectionBitMap::Right => ViewableDirectionBitMap::Back,
            //     ViewableDirectionBitMap::Front => ViewableDirectionBitMap::Left,
            //     ViewableDirectionBitMap::Back => ViewableDirectionBitMap::Right,
            // },
            Rotate::Deg180 => self.clone(),
            Rotate::Deg90 | Rotate::Deg270 => match self {
                ViewableDirectionBitMap::Top => ViewableDirectionBitMap::Bottom,
                ViewableDirectionBitMap::Bottom => ViewableDirectionBitMap::Top,
                ViewableDirectionBitMap::Left => ViewableDirectionBitMap::Back,
                ViewableDirectionBitMap::Right => ViewableDirectionBitMap::Front,
                ViewableDirectionBitMap::Front => ViewableDirectionBitMap::Right,
                ViewableDirectionBitMap::Back => ViewableDirectionBitMap::Left,
            },
            _ => {
                warn!("Rotate not implemented");
                *self
            }
        }
    }
}

impl ViewableDirection {
    pub fn has_flag(&self, flag: ViewableDirectionBitMap) -> bool {
        let target: u8 = flag as u8;
        return (self.0 & target) == target;
    }

    pub fn add_flag(&mut self, flag: ViewableDirectionBitMap) {
        self.0 += flag as u8;
    }
}

/// A sequqential direction enum used for indexing arrays
#[derive(Clone, Copy, PartialEq, Debug, Deserialize, Serialize)]
pub enum AxisAlignedDirection {
    Top = 0,
    Bottom = 1,
    Left = 2,
    Right = 3,
    Front = 4,
    Back = 5,
}

impl From<ViewableDirectionBitMap> for AxisAlignedDirection {
    fn from(value: ViewableDirectionBitMap) -> Self {
        match value {
            ViewableDirectionBitMap::Top => AxisAlignedDirection::Top,
            ViewableDirectionBitMap::Bottom => AxisAlignedDirection::Bottom,
            ViewableDirectionBitMap::Left => AxisAlignedDirection::Left,
            ViewableDirectionBitMap::Right => AxisAlignedDirection::Right,
            ViewableDirectionBitMap::Front => AxisAlignedDirection::Front,
            ViewableDirectionBitMap::Back => AxisAlignedDirection::Back,
        }
    }
}

pub fn calculate_viewable(
    block_states: &BlockStates,
    chunk: &ChunkData,
    block: &Block,
    pos: [usize; 3],
) -> ViewableDirection {
    let world = &chunk.world;

    let mut direction: u8 = 0;

    if pos[1] != world[0].len() - 1
        && should_draw_betweens(block_states, world, pos, [0, 1, 0], block)
    {
        direction += ViewableDirectionBitMap::Top as u8;
    }

    if pos[1] != 0 && should_draw_betweens(block_states, world, pos, [0, -1, 0], block) {
        direction += ViewableDirectionBitMap::Bottom as u8;
    }

    if pos[0] != world.len() - 1 && should_draw_betweens(block_states, world, pos, [1, 0, 0], block)
    {
        direction += ViewableDirectionBitMap::Right as u8;
    }

    if pos[0] != 0 && should_draw_betweens(block_states, world, pos, [-1, 0, 0], block) {
        direction += ViewableDirectionBitMap::Left as u8;
    }

    if pos[2] != world[0][0].len() - 1
        && should_draw_betweens(block_states, world, pos, [0, 0, 1], block)
    {
        direction += ViewableDirectionBitMap::Back as u8;
    }

    if pos[2] != 0 && should_draw_betweens(block_states, world, pos, [0, 0, -1], block) {
        direction += ViewableDirectionBitMap::Front as u8;
    }

    ViewableDirection(direction)
}

fn should_draw_betweens(
    block_states: &BlockStates,
    world: &RawChunkData,
    pos: [usize; 3],
    offset: [isize; 3],
    src_block: &Block,
) -> bool {
    let block_id = world[(pos[0] as isize + offset[0]) as usize]
        [(pos[1] as isize + offset[1]) as usize][(pos[2] as isize + offset[2]) as usize];

    if block_id == 0 {
        return true;
    }

    let block = block_states.get_block(block_id as usize);

    // If its the same block we don't want borders drawn between them, or if they're both waterlogged
    if (block.translucent) && block.identifier == src_block.identifier {
        return block.draw_betweens;
    }
    if !block.full {
        return true;
    }

    block.translucent
}
