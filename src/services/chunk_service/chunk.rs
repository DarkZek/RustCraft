use crate::block::Block;
use wgpu::{Buffer, BindGroup};
use crate::services::chunk_service::mesh::Vertex;
use crate::services::settings_service::{CHUNK_SIZE, CHUNK_HEIGHT};
use crate::services::chunk_service::mesh::culling::ViewableDirection;

pub struct Chunk {
    pub world: RawChunkData,
    pub blocks: Vec<Block>,
    pub vertices: Option<Vec<Vertex>>,
    pub indices: Option<Vec<u16>>,
    pub vertices_buffer: Option<Buffer>,
    pub indices_buffer: Option<Buffer>,
    pub indices_buffer_len: u32,
    pub model_bind_group: Option<BindGroup>,
    pub viewable_map: Option<[[[ViewableDirection; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE]>,
    pub x: i32,
    pub z: i32,
}

impl Chunk {
    pub fn new(data: ChunkData, chunk_coords: [i32; 2]) -> Chunk {
        Chunk {
            world: data.0,
            blocks: data.1,
            vertices: None,
            indices: None,
            vertices_buffer: None,
            indices_buffer: None,
            indices_buffer_len: 0,
            model_bind_group: None,
            viewable_map: None,
            x: chunk_coords[0],
            z: chunk_coords[1],
        }
    }
}

pub type RawChunkData = [[[u32; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE];
pub type ChunkData = (RawChunkData, Vec<Block>);