use std::collections::HashMap;
use fnv::FnvBuildHasher;
use crate::systems::chunk::data::ChunkData;
use crate::systems::chunk::ChunkSystem;
use nalgebra::Vector3;


/// By using this lookup index using relative chunk positions from the first chunk we can lookup chunks with O(1) speed
const NEARBY_CHUNK_LOOKUP_INDEX: [[[usize; 3]; 3]; 3] = [
    [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
    [[10, 11, 12], [13, 0, 14], [15, 16, 17]],
    [[18, 19, 20], [21, 22, 23], [24, 25, 26]],
];

/// A cache object that allows accessing of chunk data in an absolute format by caching nearby chunks
pub struct NearbyChunkCache<'a> {
    pos: Vector3<i32>,
    nearby: [Option<&'a ChunkData>; 9 + 9 + 9],
}

impl<'a> NearbyChunkCache<'a> {

    pub fn position(&self) -> Vector3<i32> {
        self.pos
    }

    pub fn empty(pos: Vector3<i32>) -> NearbyChunkCache<'a> {
        NearbyChunkCache {
            pos,
            nearby: [None; 9+9+9],
        }
    }

    pub fn from_service(service: &'a ChunkSystem, chunk: Vector3<i32>) -> NearbyChunkCache<'a> {
        Self::from_map(&service.chunks, chunk)
    }


    pub fn from_map(chunks: &'a HashMap<Vector3<i32>, ChunkData, FnvBuildHasher>, chunk: Vector3<i32>) -> NearbyChunkCache<'a> {
        let mut nearby: [Option<&'a ChunkData>; 9 * 3] = [None; 9 * 3];

        nearby[0] = chunks.get(&chunk);

        let mut i = 1;

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }

                    nearby[i] = chunks.get(&(chunk + Vector3::new(x, y, z)));
                    i += 1;
                }
            }
        }

        NearbyChunkCache { pos: chunk, nearby }
    }

    pub fn set_chunk(&mut self, chunk: &'a ChunkData) {
        let pos = chunk.position - self.pos;
        if pos.x < -1 || pos.x > 1 || pos.y < -1 || pos.y > 1 || pos.z < -1 || pos.z > 1 {
            panic!("Incorrectly set chunk");
        }

        self.nearby[NEARBY_CHUNK_LOOKUP_INDEX[(pos.x + 1) as usize][(pos.y + 1) as usize]
            [(pos.z + 1) as usize]] = Some(chunk);
    }

    pub fn get_chunk(&self, pos: Vector3<i32>) -> Option<&'a ChunkData> {
        // Relative pos
        let pos = pos - self.pos;

        self.get_relative_chunk(pos)
    }

    pub fn get_relative_chunk(&self, pos: Vector3<i32>) -> Option<&'a ChunkData> {
        if pos.x < -1 || pos.x > 1 || pos.y < -1 || pos.y > 1 || pos.z < -1 || pos.z > 1 {
            return None;
        }

        self.nearby[NEARBY_CHUNK_LOOKUP_INDEX[(pos.x + 1) as usize][(pos.y + 1) as usize]
            [(pos.z + 1) as usize]]
    }
}
