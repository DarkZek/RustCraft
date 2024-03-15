use crate::CHUNK_SIZE;
use nalgebra::Vector3;
use std::collections::HashMap;

pub type RawChunkData = [[[u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
pub type RawLightingData = [[[LightingColor; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

pub type Metadata = serde_json::Value;

pub type ChunkBlockMetadata = HashMap<Vector3<u8>, HashMap<String, Metadata>>;
pub type ChunkMetadata = HashMap<String, Metadata>;

pub type LightingColor = [u8; 4];

pub trait ChunkSystemTrait {
    fn get_raw_chunk(&self, pos: &Vector3<i32>) -> Option<&RawChunkData>;
    fn get_raw_chunk_mut(&mut self, pos: &Vector3<i32>) -> Option<&mut RawChunkData>;
}
