use bevy::ecs::component::Component;
use bevy::prelude::{Entity, Handle, Mesh};
use nalgebra::Vector3;
use rc_shared::chunk::{RawChunkData, RawLightingData};
use rc_shared::viewable_direction::ViewableDirection;
use rc_shared::CHUNK_SIZE;

pub mod viewable;

#[derive(Debug, Component, Clone, PartialEq)]
pub struct ChunkData {
    pub position: Vector3<i32>,

    pub world: RawChunkData,

    // TODO: Investigate not storing this
    pub viewable_map: Option<[[[ViewableDirection; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,

    // Stores the lighting intensity and color map
    pub light_levels: RawLightingData,

    // Always set except for during tests
    pub handles: Option<ChunkHandleData>
}

impl ChunkData {
    pub fn new(
        data: RawChunkData,
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
            light_levels: [[[[255, 255, 255, 255]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            handles: Some(ChunkHandleData {
                entity,
                opaque_entity,
                transparent_entity,
                opaque_mesh,
                translucent_mesh,
            })
        }
    }

    pub fn new_handleless(
        data: RawChunkData,
        position: Vector3<i32>,
    ) -> ChunkData {
        ChunkData {
            world: data,
            viewable_map: None,
            position,
            light_levels: [[[[255, 255, 255, 255]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            handles: None
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