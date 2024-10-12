use std::collections::HashMap;
use fnv::FnvBuildHasher;
use crate::systems::chunk::ChunkSystem;
use nalgebra::Vector2;
use rc_shared::chunk_column::ChunkColumnData;

/// By using this lookup index using relative chunk positions from the first chunk we can lookup chunks with O(1) speed
const NEARBY_CHUNK_LOOKUP_INDEX: [[usize; 3]; 3] = [
    [1, 2, 3],
    [4, 0, 5],
    [6, 7, 8],
];

/// A cache object that allows accessing of chunk data in an absolute format by caching nearby chunks
pub struct NearbyChunkColumnCache<'a> {
    pos: Vector2<i32>,
    nearby: [Option<&'a ChunkColumnData>; 9],
}

impl<'a> NearbyChunkColumnCache<'a> {

    pub fn position(&self) -> Vector2<i32> {
        self.pos
    }

    pub fn empty(pos: Vector2<i32>) -> NearbyChunkColumnCache<'a> {
        NearbyChunkColumnCache {
            pos,
            nearby: [None; 9],
        }
    }

    pub fn from_service(service: &'a ChunkSystem, chunk: Vector2<i32>) -> NearbyChunkColumnCache<'a> {
        Self::from_map(&service.chunk_columns, chunk)
    }


    pub fn from_map(chunks: &'a HashMap<Vector2<i32>, ChunkColumnData, FnvBuildHasher>, chunk: Vector2<i32>) -> NearbyChunkColumnCache<'a> {
        let mut nearby: [Option<&'a ChunkColumnData>; 9] = [None; 9];

        nearby[0] = chunks.get(&chunk);

        let mut i = 1;

        for x in -1..=1 {
            for z in -1..=1 {
                if x == 0 && z == 0 {
                    continue;
                }

                nearby[i] = chunks.get(&(chunk + Vector2::new(x, z)));
                i += 1;
            }
        }

        NearbyChunkColumnCache { pos: chunk, nearby }
    }

    pub fn get_chunk(&self, pos: Vector2<i32>) -> Option<&'a ChunkColumnData> {
        // Relative pos
        let pos = pos - self.pos;

        self.get_relative_chunk(pos)
    }

    pub fn get_relative_chunk(&self, pos: Vector2<i32>) -> Option<&'a ChunkColumnData> {
        if pos.x < -1 || pos.x > 1 || pos.y < -1 || pos.y > 1 {
            return None;
        }
        //
        // let Some(Some(index)) = NEARBY_CHUNK_LOOKUP_INDEX
        //     .get((pos.x + 1) as usize)
        //     .map(|v| v.get((pos.y + 1) as usize)) else {
        //     return None
        // };
        //
        // self.nearby[*index]

        self.nearby[NEARBY_CHUNK_LOOKUP_INDEX[(pos.x + 1) as usize][(pos.y + 1) as usize]]
    }
}
