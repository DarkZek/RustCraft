use nalgebra::Vector3;
use rc_shared::chunk::{ChunkPosition, RawChunkData};
use rc_shared::helpers::global_to_local_position;

pub mod tree;

#[derive(Copy, Clone, Debug)]
pub struct StructureBoundingBox {
    pub bottom_left: Vector3<i32>,
    pub size: Vector3<i32>
}

impl StructureBoundingBox {
    pub fn new(
        bottom_left: Vector3<i32>,
        size: Vector3<i32>
    ) -> Self {
        StructureBoundingBox {
            bottom_left,
            size,
        }
    }

    pub fn collides(&self, other: &StructureBoundingBox) -> bool {
        let x_check_1 = other.bottom_left.x >= self.bottom_left.x + self.size.x;
        let x_check_2 = other.bottom_left.x + other.size.x <= self.bottom_left.x;

        let y_check_1 = other.bottom_left.y >= self.bottom_left.y + self.size.y;
        let y_check_2 = other.bottom_left.y + other.size.y <= self.bottom_left.y;

        let z_check_1 = other.bottom_left.z >= self.bottom_left.z + self.size.z;
        let z_check_2 = other.bottom_left.z + other.size.z <= self.bottom_left.z;

        !(x_check_1 || x_check_2 || y_check_1 || y_check_2 || z_check_1 || z_check_2)
    }

    pub fn shifted(mut self, movement: Vector3<i32>) -> Self {
        self.bottom_left += movement;
        self
    }
}

pub trait StructureGenerator {
    fn bounding_box() -> StructureBoundingBox;
    fn spawn(seed: u32, chunk_pos: Vector3<i32>, pos: Vector3<i32>, world: &mut RawChunkData);
}

fn try_place_block(
    affected_chunk_pos: ChunkPosition,
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