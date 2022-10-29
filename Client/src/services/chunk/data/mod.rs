use crate::game::viewable_direction::ViewableDirection;
use crate::Entity;
use bevy::ecs::component::Component;

use rustcraft_protocol::constants::CHUNK_SIZE;
use nalgebra::Vector3;

pub mod generate_mesh;
pub mod viewable;

#[derive(Debug, Component)]
pub struct ChunkData {
    pub position: Vector3<i32>,

    pub entity: Entity,

    pub world: RawChunkData,

    // TODO: Investigate if caching this is even faster
    pub viewable_map: Option<[[[ViewableDirection; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,

    // Stores the lighting intensity and color map
    pub light_levels: RawLightingData,
}

impl ChunkData {
    pub fn new(data: RawChunkData, entity: Entity, position: Vector3<i32>) -> ChunkData {
        ChunkData {
            world: data,
            viewable_map: None,
            position,
            light_levels: [[[[0, 0, 0, 255]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            entity,
        }
    }
}

pub type RawChunkData = [[[u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
pub type RawLightingData = [[[Color; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

pub type Color = [u8; 4];
