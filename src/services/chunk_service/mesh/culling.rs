use crate::block::Block;
use crate::services::chunk_service::chunk::{Chunk, RawChunkData};
use crate::services::chunk_service::mesh::ViewableDirectionBitMap;

#[derive(Copy, Clone)]
pub struct ViewableDirection(pub u8);

impl ViewableDirection {
    pub fn has_flag(&self, flag: ViewableDirectionBitMap) -> bool {
        let target: u8 = flag as u8;
        return (self.0 & target) == target;
    }

    pub fn add_flag(&mut self, flag: ViewableDirectionBitMap) {
        self.0 += flag as u8;
    }
}

pub fn calculate_viewable(chunk: &Chunk, pos: [usize; 3]) -> ViewableDirection {
    let world = &chunk.world;
    let mut direction: u8 = 0;

    if pos[1] != world.as_ref().unwrap()[0].len() - 1
        && is_offset_transparent(world.as_ref().unwrap(), pos, &chunk.blocks, [0, 1, 0])
    {
        direction += ViewableDirectionBitMap::Top as u8;
    }

    if pos[1] != 0 && is_offset_transparent(world.as_ref().unwrap(), pos, &chunk.blocks, [0, -1, 0])
    {
        direction += ViewableDirectionBitMap::Bottom as u8;
    }

    if pos[0] != world.as_ref().unwrap().len() - 1
        && is_offset_transparent(world.as_ref().unwrap(), pos, &chunk.blocks, [1, 0, 0])
    {
        direction += ViewableDirectionBitMap::Right as u8;
    }

    if pos[0] != 0 && is_offset_transparent(world.as_ref().unwrap(), pos, &chunk.blocks, [-1, 0, 0])
    {
        direction += ViewableDirectionBitMap::Left as u8;
    }

    if pos[2] != world.as_ref().unwrap()[0][0].len() - 1
        && is_offset_transparent(world.as_ref().unwrap(), pos, &chunk.blocks, [0, 0, 1])
    {
        direction += ViewableDirectionBitMap::Back as u8;
    }

    if pos[2] != 0 && is_offset_transparent(world.as_ref().unwrap(), pos, &chunk.blocks, [0, 0, -1])
    {
        direction += ViewableDirectionBitMap::Front as u8;
    }

    ViewableDirection(direction)
}

fn is_offset_transparent(
    world: &RawChunkData,
    pos: [usize; 3],
    blocks: &Vec<Block>,
    offset: [isize; 3],
) -> bool {
    let block_id = world[(pos[0] as isize + offset[0]) as usize]
        [(pos[1] as isize + offset[1]) as usize][(pos[2] as isize + offset[2]) as usize];

    if block_id == 0 {
        //TODO: Change this back to true
        return true;
    }

    blocks.get(block_id as usize - 1).unwrap().transparent
}
