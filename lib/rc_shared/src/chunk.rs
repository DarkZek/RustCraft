use crate::{CHUNK_SIZE, MAX_LIGHT_VALUE};
use nalgebra::{Vector2, Vector3};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::block::BlockId;

pub type RawChunkData = [[[u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
pub type RawLightingData = [[[LightingColor; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

pub type ChunkPosition = Vector3<i32>;
pub type ChunkColumnPosition = Vector2<i32>;
pub type LocalBlockPosition = Vector3<usize>;
pub type GlobalBlockPosition = Vector3<i32>;

pub type Metadata = serde_json::Value;

pub type ChunkBlockMetadata = HashMap<Vector3<u8>, HashMap<String, Metadata>>;
pub type ChunkMetadata = HashMap<String, Metadata>;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct LightingColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub strength: u8,
    pub skylight: u8
}

impl LightingColor {
    pub fn full() -> LightingColor {
        LightingColor {
            r: 255,
            g: 255,
            b: 255,
            strength: MAX_LIGHT_VALUE as u8,
            skylight: 0,
        }
    }
}

pub trait ChunkSystemTrait {
    fn get_raw_chunk(&self, pos: &Vector3<i32>) -> Option<&ChunkDataStorage>;
    fn get_raw_chunk_mut(&mut self, pos: &Vector3<i32>) -> Option<&mut ChunkDataStorage>;
}


/// Chunk data in a format that allows for more compact storage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChunkDataStorage {
    /// The chunk contains data
    Data(Box<RawChunkData>),

    /// The chunk contains no blocks other than empty
    Empty
}

impl ChunkDataStorage {
    #[inline]
    pub fn get(&self, pos: LocalBlockPosition) -> BlockId {
        match self {
            ChunkDataStorage::Data(data) => {
                data[pos.x][pos.y][pos.z]
            },
            // Every block is empty
            ChunkDataStorage::Empty => 0
        }
    }

    #[inline]
    pub fn set(&mut self, pos: LocalBlockPosition, id: BlockId) {
        match self {
            ChunkDataStorage::Data(_) => {}
            ChunkDataStorage::Empty => {
                // If the target block is air, the entire chunk is already air so return
                if id == 0 {
                    return;
                }

                // Convert `self` to a storage where we can change individual blocks
                *self = ChunkDataStorage::Data(Box::new([[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]))
            }
        }

        let ChunkDataStorage::Data(data) = self else { panic!("Impossible"); };

        data[pos.x][pos.y][pos.z] = id;
    }

    /// Shrinks the data down to the smallest storage possible
    pub fn optimise(&mut self) {

        // This is already the most optimised form
        if ChunkDataStorage::Empty == *self {
            return
        }

        let mut is_all_air = true;

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let id = self.get(Vector3::new(x, y, z));

                    if id != 0 {
                        is_all_air = false;
                    }
                }
            }
        }

        if is_all_air {
            *self = ChunkDataStorage::Empty;
        }
    }
}