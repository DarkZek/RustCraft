use nalgebra::Vector3;
use rc_shared::chunk::RawChunkData;
use crate::game::generation::structures::{StructureBoundingBox, StructureGenerator, try_place_block};

pub struct TreeStructureGenerator;

impl StructureGenerator for TreeStructureGenerator {
    fn bounding_box() -> StructureBoundingBox {
        StructureBoundingBox {
            bottom_left: Vector3::new(-1, 0, -1),
            size: Vector3::new(3, 6, 3)
        }
    }

    fn spawn(seed: u32, chunk_pos: Vector3<i32>, pos: Vector3<i32>, world: &mut RawChunkData) {
        try_place_block(chunk_pos, world, pos, 4);
        try_place_block(chunk_pos, world, pos + Vector3::new(0, 1, 0), 4);
        try_place_block(chunk_pos, world, pos + Vector3::new(0, 2, 0), 4);
        try_place_block(chunk_pos, world, pos + Vector3::new(0, 3, 0), 4);

        try_place_block(chunk_pos, world, pos + Vector3::new(1, 3, 0), 5);
        try_place_block(chunk_pos, world, pos + Vector3::new(1, 3, -1), 5);
        try_place_block(chunk_pos, world, pos + Vector3::new(1, 3, 1), 5);
        try_place_block(chunk_pos, world, pos + Vector3::new(0, 3, 1), 5);
        try_place_block(chunk_pos, world, pos + Vector3::new(0, 3, -1), 5);
        try_place_block(chunk_pos, world, pos + Vector3::new(-1, 3, 0), 5);
        try_place_block(chunk_pos, world, pos + Vector3::new(-1, 3, -1), 5);
        try_place_block(chunk_pos, world, pos + Vector3::new(-1, 3, 1), 5);

        try_place_block(chunk_pos, world, pos + Vector3::new(1, 4, 0), 5);
        try_place_block(chunk_pos, world, pos + Vector3::new(1, 4, -1), 5);
        try_place_block(chunk_pos, world, pos + Vector3::new(1, 4, 1), 5);
        try_place_block(chunk_pos, world, pos + Vector3::new(0, 4, 1), 5);
        try_place_block(chunk_pos, world, pos + Vector3::new(0, 4, -1), 5);
        try_place_block(chunk_pos, world, pos + Vector3::new(0, 4, 0), 5);
        try_place_block(chunk_pos, world, pos + Vector3::new(-1, 4, 0), 5);
        try_place_block(chunk_pos, world, pos + Vector3::new(-1, 4, -1), 5);
        try_place_block(chunk_pos, world, pos + Vector3::new(-1, 4, 1), 5);

        try_place_block(chunk_pos, world, pos + Vector3::new(0, 5, 0), 5);
    }
}