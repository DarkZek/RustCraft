use crate::block::blocks::BLOCK_STATES;
use crate::block::Block;
use crate::services::chunk_service::chunk::{ChunkData, RawChunkData};
use crate::services::chunk_service::mesh::ViewableDirectionBitMap;

#[derive(Copy, Clone, Debug)]
pub struct ViewableDirection(pub u8);

impl ViewableDirection {
    pub fn has_flag(&self, flag: &ViewableDirectionBitMap) -> bool {
        let target: u8 = *flag as u8;
        return (self.0 & target) == target;
    }

    pub fn add_flag(&mut self, flag: ViewableDirectionBitMap) {
        self.0 += flag as u8;
    }
}

pub fn calculate_viewable(
    chunk: &ChunkData,
    block: &Option<Block>,
    pos: [usize; 3],
) -> ViewableDirection {
    let world = &chunk.world;

    let mut direction: u8 = 0;

    if pos[1] != world[0].len() - 1 && should_draw_betweens(world, pos, [0, 1, 0], &block) {
        direction += ViewableDirectionBitMap::Top as u8;
    }

    if pos[1] != 0 && should_draw_betweens(world, pos, [0, -1, 0], &block) {
        direction += ViewableDirectionBitMap::Bottom as u8;
    }

    if pos[0] != world.len() - 1 && should_draw_betweens(world, pos, [1, 0, 0], &block) {
        direction += ViewableDirectionBitMap::Right as u8;
    }

    if pos[0] != 0 && should_draw_betweens(world, pos, [-1, 0, 0], &block) {
        direction += ViewableDirectionBitMap::Left as u8;
    }

    if pos[2] != world[0][0].len() - 1 && should_draw_betweens(world, pos, [0, 0, 1], &block) {
        direction += ViewableDirectionBitMap::Back as u8;
    }

    if pos[2] != 0 && should_draw_betweens(world, pos, [0, 0, -1], &block) {
        direction += ViewableDirectionBitMap::Front as u8;
    }

    ViewableDirection(direction)
}

fn should_draw_betweens(
    world: &RawChunkData,
    pos: [usize; 3],
    offset: [isize; 3],
    src_block: &Option<Block>,
) -> bool {
    let block_id = world[(pos[0] as isize + offset[0]) as usize]
        [(pos[1] as isize + offset[1]) as usize][(pos[2] as isize + offset[2]) as usize];

    if block_id == 0 {
        return true;
    }

    match BLOCK_STATES.get() {
        None => {
            log_error!("Blockstates list was not generated");
            false
        }
        Some(states) => match states.get_block(block_id as usize) {
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
                    if block.block_type.is_waterlogged()
                        && src_block.as_ref().unwrap().block_type.is_waterlogged()
                    {
                        return false;
                    }
                    if block.block_type.get_transparency()
                        && block.block_type.get_identifier()
                            == src_block.as_ref().unwrap().block_type.get_identifier()
                    {
                        return true;
                    }
                }
                block.block_type.get_transparency()
            }
        },
    }
}
