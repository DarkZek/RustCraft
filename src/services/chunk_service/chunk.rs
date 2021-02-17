use crate::block::Block;
use crate::services::chunk_service::mesh::culling::ViewableDirection;
use crate::services::chunk_service::mesh::{MeshData, Vertex};
use crate::services::settings_service::CHUNK_SIZE;
use nalgebra::Vector3;
use std::collections::HashMap;
use wgpu::{BindGroup, Buffer};

pub struct Chunks(pub HashMap<Vector3<i32>, Chunk>);

impl Default for Chunks {
    fn default() -> Self {
        unimplemented!()
    }
}

pub enum Chunk {
    Tangible(ChunkData),
    Intangible,
}

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
