use crate::CHUNK_SIZE;
use nalgebra::Vector3;

pub type RawChunkData = [[[u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
pub type RawLightingData = [[[LightingColor; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

pub type LightingColor = [u8; 4];

pub trait ChunkSystemTrait {
    fn get_raw_chunk<'a>(&'a self, pos: &Vector3<i32>) -> Option<&'a RawChunkData>;
}
