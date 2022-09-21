use crate::game::blocks::{Block, BlockStates};
use crate::services::asset::atlas::index::Rotate;
use crate::services::chunk::data::{ChunkData, RawChunkData};
use crate::warn;
use bevy::log::error;
use nalgebra::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct ViewableDirection(pub u8);

#[derive(Clone, Copy, PartialEq)]
pub enum ViewableDirectionBitMap {
    Top = 0b00000001,
    Bottom = 0b00000010,
    Left = 0b00000100,
    Right = 0b00001000,
    Front = 0b00010000,
    Back = 0b00100000,
}

impl ViewableDirectionBitMap {
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

pub fn calculate_viewable(
    block_states: &BlockStates,
    chunk: &ChunkData,
    block: &Option<Box<dyn Block>>,
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
    src_block: &Option<Box<dyn Block>>,
) -> bool {
    let block_id = world[(pos[0] as isize + offset[0]) as usize]
        [(pos[1] as isize + offset[1]) as usize][(pos[2] as isize + offset[2]) as usize];

    if block_id == 0 {
        return true;
    }

    match block_states.get_block(block_id as usize) {
        None => {
            // log_error!(format!(
            //     "Block with invalid blockstate: X {} Y {} Z {} Block ID {}",
            //     pos[0], pos[1], pos[2], block_id
            // ));
            false
        }
        Some(block) => {
            // If its the same block we don't want borders drawn between them, or if they're both waterlogged
            if src_block.is_some() {
                if (block.is_translucent())
                    && block.identifier() == src_block.as_ref().unwrap().identifier()
                {
                    return block.draw_betweens();
                }
                if !block.is_full() {
                    return true;
                }
            }
            block.is_translucent()
        }
    }
}
