use crate::game::viewable_direction::ViewableDirection;
use bevy::ecs::component::Component;
use bevy::prelude::{Entity, Handle, Mesh};

use nalgebra::Vector3;
use rc_networking::constants::CHUNK_SIZE;

pub mod viewable;

#[derive(Debug, Component)]
pub struct ChunkData {
    pub position: Vector3<i32>,

    pub entity: Entity,
    pub opaque_entity: Entity,
    pub transparent_entity: Entity,

    pub opaque_mesh: Handle<Mesh>,
    pub translucent_mesh: Handle<Mesh>,

    pub world: RawChunkData,

    pub viewable_map: Option<[[[ViewableDirection; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,

    // Stores the lighting intensity and color map
    pub light_levels: RawLightingData,
}

impl ChunkData {
    pub fn new(
        data: RawChunkData,
        entity: Entity,
        opaque_entity: Entity,
        transparent_entity: Entity,
        position: Vector3<i32>,
        opaque_mesh: Handle<Mesh>,
        translucent_mesh: Handle<Mesh>,
    ) -> ChunkData {
        ChunkData {
            world: data,
            viewable_map: None,
            position,
            light_levels: [[[[255, 255, 255, 255]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            entity,
            opaque_entity,
            transparent_entity,
            opaque_mesh,
            translucent_mesh,
        }
    }
}

pub type RawChunkData = [[[u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
pub type RawLightingData = [[[LightingColor; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

pub type LightingColor = [u8; 4];
