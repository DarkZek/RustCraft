use crate::block::Block;
use crate::render::mesh::Vertex;
use wgpu::{Buffer, BindGroup};
use crate::world::CHUNK_SIZE;

pub struct Chunk {
    pub world: ChunkData,
    pub blocks: Vec<Block>,
    pub vertices: Option<Vec<Vertex>>,
    pub indices: Option<Vec<u16>>,
    pub vertices_buffer: Option<Buffer>,
    pub indices_buffer: Option<Buffer>,
    pub indices_buffer_len: u32,
    pub model_bind_group: Option<BindGroup>,
    pub x: i32,
    pub z: i32,
}

pub type ChunkData = [[[u32; CHUNK_SIZE]; 256]; CHUNK_SIZE];