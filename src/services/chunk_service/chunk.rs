use crate::services::chunk_service::mesh::culling::ViewableDirection;
use crate::services::chunk_service::mesh::MeshData;
use crate::services::settings_service::CHUNK_SIZE;
use fnv::{FnvBuildHasher, FnvHashMap};
use nalgebra::Vector3;
use specs::{Component, Entity, VecStorage};
use std::collections::HashMap;
use wgpu::BindGroup;

pub struct ChunkEntityLookup {
    pub map: HashMap<Vector3<i32>, Entity, FnvBuildHasher>,
}

impl Default for ChunkEntityLookup {
    fn default() -> Self {
        ChunkEntityLookup {
            map: FnvHashMap::default(),
        }
    }
}

impl Component for ChunkEntityLookup {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct ChunkData {
    pub world: RawChunkData,

    // Opaque chunk data
    pub opaque_model: MeshData,

    // Translucent chunk data
    pub translucent_model: MeshData,

    pub model_bind_group: Option<BindGroup>,
    // TODO: Investigate if caching this is even faster
    pub viewable_map: Option<[[[ViewableDirection; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
    pub position: Vector3<i32>,

    // Stores the lighting intensity and color map
    pub light_levels: RawLightingData,

    // Stores the lighting of neighbouring chunks effect on this chunk.
    pub neighboring_light_levels: RawLightingData,
}

impl Component for ChunkData {
    type Storage = VecStorage<Self>;
}

pub type Color = [u8; 4];

impl ChunkData {
    pub fn new(data: RawChunkData, position: Vector3<i32>) -> ChunkData {
        ChunkData {
            world: data,
            opaque_model: MeshData::default(),
            translucent_model: MeshData::default(),
            model_bind_group: None,
            viewable_map: None,
            position,
            // Ambient color
            light_levels: [[[[2, 2, 2, 255]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            neighboring_light_levels: [[[[255, 255, 255, 0]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }
}

pub type RawChunkData = [[[u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
pub type RawLightingData = [[[Color; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

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
        let mut map = FnvHashMap::default();
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
