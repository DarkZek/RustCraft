use crate::block::Block;
use crate::game::physics::collider::BoxCollider;
use crate::services::chunk_service::mesh::culling::ViewableDirection;
use crate::services::chunk_service::mesh::Vertex;
use crate::services::settings_service::CHUNK_SIZE;
use nalgebra::Vector3;
use wgpu::{BindGroup, Buffer};

pub enum Chunk {
    Tangible(ChunkData),
    Intangible
}

pub struct ChunkData {
    pub world: RawChunkData,
    pub blocks: Vec<Block>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub vertices_buffer: Option<Buffer>,
    pub indices_buffer: Option<Buffer>,
    pub indices_buffer_len: u32,
    pub model_bind_group: Option<BindGroup>,
    //TODO: Investigate if caching this is even faster
    pub viewable_map: Option<[[[ViewableDirection; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
    pub position: Vector3<i32>,
    pub collision_map: Vec<BoxCollider>,

    // Stores the lighting intensity and color map
    pub light_levels: RawLightingData,

    // Stores the lighting of neighbouring chunks effect on this chunk.
    pub neighboring_light_levels: RawLightingData,
}

pub type Color = [f32; 4];

impl ChunkData {
    pub fn new(data: ChunkBlockData, position: Vector3<i32>) -> ChunkData {
        ChunkData {
            world: data.0,
            blocks: data.1,
            vertices: vec![],
            indices: vec![],
            vertices_buffer: None,
            indices_buffer: None,
            indices_buffer_len: 0,
            model_bind_group: None,
            viewable_map: None,
            position,
            collision_map: vec![],
            // Ambient color
            light_levels: [[[[1.0, 1.0, 1.0, 1.0]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            neighboring_light_levels: [[[[1.0, 1.0, 1.0, 1.0]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]
        }
    }
}

pub type RawChunkData = [[[u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
pub type RawLightingData = [[[Color; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
pub type ChunkBlockData = (RawChunkData, Vec<Block>);
