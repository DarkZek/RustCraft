use crate::game::chunk::ChunkData;

use crate::error::ServerError;
use crate::helpers::global_to_local_position;
use bevy::ecs::entity::Entity;
use bevy::ecs::prelude::Resource;
use bevy::log::error;
use bevy::prelude::info;
use nalgebra::Vector3;
use rc_networking::constants::GameObjectId;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::AtomicU64;

pub const ENTITY_ID_COUNT: AtomicU64 = AtomicU64::new(0);

#[derive(Resource)]
pub struct WorldData {
    pub chunks: HashMap<Vector3<i32>, ChunkData>,
    pub entities: HashMap<GameObjectId, Entity>,
}

impl WorldData {
    pub fn load_spawn_chunks() -> Self {
        let mut chunks = HashMap::new();

        // Load spawn area
        for x in -3..=3 {
            for y in 0..=5 {
                for z in -3..=3 {
                    let pos = Vector3::new(x, y, z);

                    let chunk = match Self::try_load_chunk(pos) {
                        Ok(Some(chunk)) => chunk,
                        Ok(None) => ChunkData::generate(pos),
                        Err(err) => {
                            error!("Error reading chunk data: {:?}", err);
                            ChunkData::generate(pos)
                        }
                    };

                    chunks.insert(Vector3::new(x, y, z), chunk);
                }
            }
        }

        info!("Loaded spawn chunks");

        WorldData {
            chunks,
            entities: Default::default(),
        }
    }

    pub fn get_block_id(&self, pos: Vector3<i32>) -> Option<u32> {
        let (chunk_pos, local_pos) = global_to_local_position(pos);

        self.chunks
            .get(&chunk_pos)
            .map(|v| v.world[local_pos.x][local_pos.y][local_pos.z])
    }

    pub fn set_block_id(&mut self, pos: Vector3<i32>, block_id: u32) -> Option<()> {
        let (chunk_pos, local_pos) = global_to_local_position(pos);

        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.world[local_pos.x][local_pos.y][local_pos.z] = block_id;
            Some(())
        } else {
            None
        }
    }

    pub fn try_load_chunk(location: Vector3<i32>) -> Result<Option<ChunkData>, ServerError> {
        let path = format!(
            "./world/{:08x}{:08x}{:08x}.chunk",
            location.x, location.y, location.z
        );
        if !fs::try_exists(&path)? {
            return Ok(None);
        }

        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);

        let chunk: ChunkData = serde_json::from_reader(&mut reader)?;

        Ok(Some(chunk))
    }
}
