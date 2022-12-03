use crate::systems::chunk::data::ChunkData;
use fnv::{FnvBuildHasher, FnvHashMap};
use nalgebra::Vector3;
use std::collections::HashMap;

pub struct Chunks<'a> {
    data: Option<Vec<&'a ChunkData>>,
    data_mut: Option<Vec<&'a mut ChunkData>>,
    map: HashMap<Vector3<i32>, usize, FnvBuildHasher>,
}

impl Chunks<'_> {
    pub fn new(data: Vec<&ChunkData>) -> Chunks {
        let mut map = FnvHashMap::default();
        for (i, chunk) in data.iter().enumerate() {
            map.insert(chunk.position, i);
        }

        Chunks {
            data: Some(data),
            data_mut: None,
            map,
        }
    }

    pub fn new_mut(data_mut: Vec<&mut ChunkData>) -> Chunks {
        let mut map =
            FnvHashMap::with_capacity_and_hasher(data_mut.len(), FnvBuildHasher::default());
        for (i, chunk) in data_mut.iter().enumerate() {
            map.insert(chunk.position, i);
        }

        Chunks {
            data: None,
            data_mut: Some(data_mut),
            map,
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn get_loc(&self, loc: Vector3<i32>) -> Option<&ChunkData> {
        if let Option::Some(index) = self.map.get(&loc) {
            Some(self.data.as_ref().unwrap()[*index])
        } else {
            None
        }
    }

    pub fn get_mut_loc(&mut self, loc: Vector3<i32>) -> Option<&mut ChunkData> {
        if let Option::Some(index) = self.map.get(&loc) {
            Some(self.data_mut.as_mut().unwrap()[*index])
        } else {
            None
        }
    }
}
