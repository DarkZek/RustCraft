use crate::block::Block;
use crate::services::chunk_service::mesh::culling::ViewableDirection;
use crate::services::chunk_service::mesh::{MeshData, Vertex};
use crate::services::settings_service::CHUNK_SIZE;
use nalgebra::Vector3;
use specs::{Component, DenseVecStorage, NullStorage, ReadStorage, VecStorage};
use std::collections::HashMap;
use std::mem::MaybeUninit;
use wgpu::{BindGroup, Buffer};

#[derive(Debug)]
pub struct ChunkData {
    pub world: RawChunkData,

    // Opaque chunk data
    pub opaque_model: MeshData,

    // Translucent chunk data
    pub translucent_model: MeshData,

    pub model_bind_group: Option<BindGroup>,
    //TODO: Investigate if caching this is even faster
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

pub struct RerenderChunkFlag {
    pub chunk: Vector3<i32>,
}
impl Component for RerenderChunkFlag {
    type Storage = DenseVecStorage<Self>;
}
impl Default for RerenderChunkFlag {
    fn default() -> Self {
        unimplemented!()
    }
}

pub type Color = [u8; 4];

impl ChunkData {
    pub fn new(data: ChunkBlockData, position: Vector3<i32>) -> ChunkData {
        ChunkData {
            world: data.0,
            opaque_model: MeshData::default(),
            translucent_model: MeshData::default(),
            model_bind_group: None,
            viewable_map: None,
            position,
            // Ambient color
            light_levels: [[[[2, 2, 2, 255]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            neighboring_light_levels: [[[[0, 0, 0, 0]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }
}

pub type RawChunkData = [[[u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
pub type RawLightingData = [[[Color; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
pub type ChunkBlockData = (RawChunkData, Vec<Block>);

pub struct Chunks<'a> {
    data: Option<&'a [MaybeUninit<ChunkData>]>,
    data_mut: Option<&'a mut [MaybeUninit<ChunkData>]>,
    map: HashMap<Vector3<i32>, usize>,
}

impl Chunks<'_> {
    pub fn new<'a>(data: &'a [MaybeUninit<ChunkData>]) -> Chunks<'a> {
        let mut map = HashMap::new();
        for (i, chunk) in data.iter().enumerate() {
            unsafe {
                map.insert((*chunk.as_ptr()).position, i);
            }
        }

        Chunks {
            data: Some(data),
            data_mut: None,
            map,
        }
    }
    pub fn new_mut<'a>(data_mut: &'a mut [MaybeUninit<ChunkData>]) -> Chunks<'a> {
        let mut map = HashMap::new();
        for (i, chunk) in data_mut.iter().enumerate() {
            unsafe {
                map.insert((*chunk.as_ptr()).position, i);
            }
        }

        Chunks {
            data: None,
            data_mut: Some(data_mut),
            map,
        }
    }

    pub fn get_loc(&self, loc: Vector3<i32>) -> Option<&ChunkData> {
        if let Option::Some(index) = self.map.get(&loc) {
            unsafe { Some(&(*self.data.unwrap()[*index].as_ptr())) }
        } else {
            None
        }
    }

    pub fn get_mut_loc(&mut self, loc: Vector3<i32>) -> Option<&mut ChunkData> {
        if let Option::Some(index) = self.map.get(&loc) {
            unsafe { Some(&mut (*self.data_mut.as_mut().unwrap()[*index].as_mut_ptr())) }
        } else {
            None
        }
    }
}
