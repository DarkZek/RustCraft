use std::collections::HashMap;
use nalgebra::Vector3;
use crate::game::chunk::ChunkData;
use crate::game::player::Player;

pub struct World {
    pub chunks: HashMap<Vector3<i32>, ChunkData>
}

impl World {
    pub fn new() -> Self {

        let mut chunks = HashMap::new();

        for x in -1..=1 {
            for z in -1..=1 {
                let mut chunk = ChunkData::generate(Vector3::new(x, 0, z));
                chunks.insert(Vector3::new(x, 0, z), chunk);
            }
        }

        World {
            chunks
        }
    }
}