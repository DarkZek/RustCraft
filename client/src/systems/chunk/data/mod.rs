use bevy::ecs::component::Component;
use bevy::prelude::{Entity, Handle, Mesh};
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use rc_shared::chunk::{ChunkDataStorage, LightingColor, RawLightingData};
use rc_shared::viewable_direction::ViewableDirection;
use rc_shared::CHUNK_SIZE;
use crate::systems::chunk::flags::ChunkFlags;

pub mod viewable;

#[derive(Debug, Component, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChunkData {
    pub position: Vector3<i32>,

    pub world: ChunkDataStorage,

    // TODO: Investigate not storing this
    pub viewable_map: Option<[[[ViewableDirection; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,

    // Stores the lighting intensity and color map
    pub light_levels: RawLightingData,

    // Always set except for during tests
    #[serde(skip)]
    pub handles: Option<ChunkHandleData>,

    pub flags: ChunkFlags
}

impl ChunkData {
    pub fn new(
        data: ChunkDataStorage,
        position: Vector3<i32>,
        entity: Entity,
        opaque_entity: Entity,
        transparent_entity: Entity,
        opaque_mesh: Handle<Mesh>,
        translucent_mesh: Handle<Mesh>,
    ) -> ChunkData {
        ChunkData {
            world: data,
            viewable_map: None,
            position,
            light_levels: [[[LightingColor::full(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            handles: Some(ChunkHandleData {
                entity,
                opaque_entity,
                transparent_entity,
                opaque_mesh,
                translucent_mesh,
            }),
            flags: Default::default(),
        }
    }

    pub fn new_handleless(
        world: ChunkDataStorage,
        position: Vector3<i32>,
    ) -> ChunkData {
        ChunkData {
            world,
            viewable_map: None,
            position,
            light_levels: [[[LightingColor::full(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            handles: None,
            flags: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChunkHandleData {
    pub entity: Entity,
    pub opaque_entity: Entity,
    pub transparent_entity: Entity,

    pub opaque_mesh: Handle<Mesh>,
    pub translucent_mesh: Handle<Mesh>,

}