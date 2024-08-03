use nalgebra::Vector3;
use rc_shared::chunk::RawChunkData;
use crate::helpers::global_to_local_position;

pub mod tree;

pub struct StructureBoundingBox {
    bottom_left: Vector3<i32>,
    size: Vector3<i32>
}

pub trait StructureGenerator {
    fn bounding_box() -> StructureBoundingBox;
    fn spawn(seed: u32, chunk_pos: Vector3<i32>, pos: Vector3<i32>, world: &mut RawChunkData);
}

fn try_place_block(
    affected_chunk_pos: Vector3<i32>,
    world: &mut RawChunkData,
    pos: Vector3<i32>,
    block: u32,
) {
    let (block_chunk_pos, block_local_pos) = global_to_local_position(pos);

    // Not same chunk
    if affected_chunk_pos != block_chunk_pos {
        return;
    }

    world[block_local_pos.x][block_local_pos.y][block_local_pos.z] = block;
}