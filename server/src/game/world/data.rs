use crate::game::chunk::ChunkData;

use crate::error::ServerError;
use crate::game::world::serialized::DeserializedChunkData;
use crate::helpers::global_to_local_position;
use bevy::ecs::entity::Entity;
use bevy::ecs::prelude::Resource;
use bevy::log::error;
use bevy::prelude::{info, Commands};
use nalgebra::Vector3;
use rc_networking::constants::GameObjectId;
use rc_shared::helpers::global_f32_to_local_position;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::AtomicU64;

pub static GAME_OBJECT_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Resource)]
pub struct WorldData {
    pub chunks: HashMap<Vector3<i32>, ChunkData>,
    pub game_objects_mapping: HashMap<GameObjectId, Entity>,
    pub game_objects_chunks: HashMap<Vector3<i32>, HashMap<GameObjectId, Entity>>,
}

impl Default for WorldData {
    fn default() -> Self {
        WorldData {
            chunks: Default::default(),
            game_objects_mapping: Default::default(),
            game_objects_chunks: Default::default(),
        }
    }
}

impl WorldData {
    pub fn insert_game_object(
        &mut self,
        game_object_id: GameObjectId,
        entity: Entity,
        chunk_pos: Vector3<i32>,
    ) -> Option<Entity> {
        if !self.game_objects_chunks.contains_key(&chunk_pos) {
            self.game_objects_chunks.insert(chunk_pos, HashMap::new());
        }
        self.game_objects_chunks
            .get_mut(&chunk_pos)
            .unwrap()
            .insert(game_object_id, entity.clone());
        self.game_objects_mapping.insert(game_object_id, entity)
    }

    pub fn get_game_object(&self, game_object_id: &GameObjectId) -> Option<Entity> {
        self.game_objects_mapping.get(&game_object_id).map(|v| *v)
    }

    pub fn remove_game_object(
        &mut self,
        game_object_id: &GameObjectId,
        chunk_pos: Vector3<i32>,
    ) -> Option<Entity> {
        self.game_objects_chunks
            .get_mut(&chunk_pos)
            .and_then(|v| v.remove(&game_object_id));
        self.game_objects_mapping.remove(&game_object_id)
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

    pub fn try_load_chunk(
        location: Vector3<i32>,
    ) -> Result<Option<DeserializedChunkData>, ServerError> {
        let path = format!(
            "./world/{:08x}{:08x}{:08x}.chunk",
            location.x, location.y, location.z
        );
        if !fs::try_exists(&path)? {
            return Ok(None);
        }

        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);

        let chunk =
            serde_json::from_reader::<&mut BufReader<File>, DeserializedChunkData>(&mut reader)?;

        Ok(Some(chunk))
    }
}
