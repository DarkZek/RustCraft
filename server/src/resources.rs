use crate::game::chunk::ChunkData;

use bevy_ecs::entity::Entity;
use nalgebra::Vector3;
use rc_client::rc_protocol::constants::EntityId;
use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use bevy_ecs::prelude::Resource;

pub const ENTITY_ID_COUNT: AtomicU64 = AtomicU64::new(0);

#[derive(Resource)]
pub struct WorldData {
    pub chunks: HashMap<Vector3<i32>, ChunkData>,
    pub entities: HashMap<EntityId, Entity>,
}

impl WorldData {
    pub fn new() -> Self {
        let mut chunks = HashMap::new();

        for x in -1..=1 {
            for z in -1..=1 {
                let chunk = ChunkData::generate(Vector3::new(x, 0, z));
                chunks.insert(Vector3::new(x, 0, z), chunk);
            }
        }

        WorldData {
            chunks,
            entities: Default::default(),
        }
    }
}
