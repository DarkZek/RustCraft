use crate::game::chunk::ChunkData;

use crate::error::ServerError;
use crate::game::world::serialized::DeserializedChunkData;
use rc_shared::helpers::global_to_local_position;
use bevy::ecs::entity::Entity;
use bevy::ecs::prelude::Resource;

use nalgebra::{Vector2, Vector3};
use rc_shared::constants::GameObjectId;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::AtomicU64;
use rc_shared::block::BlockId;
use rc_shared::chunk::{ChunkColumnPosition, ChunkPosition, GlobalBlockPosition};
use rc_shared::chunk_column::ChunkColumnData;

pub static GAME_OBJECT_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Resource)]
pub struct WorldData {
    pub chunks: HashMap<ChunkPosition, ChunkData>,
    pub chunks_columns: HashMap<ChunkColumnPosition, ChunkColumnData>,
    pub game_objects_mapping: HashMap<GameObjectId, Entity>,
    pub game_objects_chunks: HashMap<ChunkPosition, HashMap<GameObjectId, Entity>>,
}

impl Default for WorldData {
    fn default() -> Self {
        WorldData {
            chunks: Default::default(),
            chunks_columns: Default::default(),
            game_objects_mapping: Default::default(),
            game_objects_chunks: Default::default(),
        }
    }
}

impl WorldData {
    pub fn insert_chunk(
        &mut self,
        chunk: ChunkData
    ) {
        let column = Vector2::new(chunk.position.x, chunk.position.z);
        self.chunks.insert(chunk.position, chunk);
        self.update_column(column);
    }

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

    pub fn get_block_id(&self, pos: GlobalBlockPosition) -> Option<u32> {
        let (chunk_pos, local_pos) = global_to_local_position(pos);

        self.chunks
            .get(&chunk_pos)
            .map(|v| v.world.get(local_pos))
    }

    pub fn set_block_id(&mut self, pos: GlobalBlockPosition, block_id: BlockId) -> Option<()> {
        let (chunk_pos, local_pos) = global_to_local_position(pos);

        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.world.set(local_pos, block_id);
            Some(())
        } else {
            None
        }
    }

    pub fn try_load_chunk(
        location: ChunkPosition,
    ) -> Result<Option<DeserializedChunkData>, ServerError> {
        let path = format!(
            "./world/chunks/{:08x}{:08x}{:08x}.chunk",
            location.x, location.y, location.z
        );
        if !fs::exists(&path)? {
            return Ok(None);
        }

        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);

        let chunk =
            serde_json::from_reader::<&mut BufReader<File>, DeserializedChunkData>(&mut reader)?;

        Ok(Some(chunk))
    }
}
