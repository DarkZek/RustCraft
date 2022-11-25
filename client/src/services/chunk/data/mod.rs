use crate::game::viewable_direction::ViewableDirection;
use bevy::ecs::component::Component;
use bevy::prelude::{Entity, Handle, Mesh};

use nalgebra::Vector3;
use rc_networking::constants::CHUNK_SIZE;

pub mod generate_mesh;
pub mod viewable;

#[derive(Debug, Component)]
pub struct ChunkData {
    pub position: Vector3<i32>,

    pub entity: Entity,

    pub mesh: Option<Handle<Mesh>>,

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
            light_levels: [[[[255, 255, 255, 255]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            entity,
            mesh: None,
        }
    }
}

pub type RawChunkData = [[[u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
pub type RawLightingData = [[[LightingColor; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

pub type LightingColor = [u8; 4];
