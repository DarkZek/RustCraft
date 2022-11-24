use crate::game::chunk::ChunkData;

use crate::error::ServerError;
use bevy::ecs::entity::Entity;
use bevy::ecs::prelude::Resource;
use bevy::log::error;
use nalgebra::Vector3;
use rc_networking::constants::EntityId;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::AtomicU64;

pub const ENTITY_ID_COUNT: AtomicU64 = AtomicU64::new(0);

#[derive(Resource)]
pub struct WorldData {
    pub chunks: HashMap<Vector3<i32>, ChunkData>,
    pub entities: HashMap<EntityId, Entity>,
}

impl WorldData {
    pub fn load_spawn_chunks() -> Self {
        let mut chunks = HashMap::new();

        // Load spawn area
        for x in -2..=2 {
            for y in 0..=3 {
                for z in -2..=2 {
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

        WorldData {
            chunks,
            entities: Default::default(),
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
