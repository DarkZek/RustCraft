use crate::systems::asset::AssetService;
use crate::systems::chunk::builder::{mesh_builder, RerenderChunkFlag, RerenderChunkFlagContext};
use crate::systems::chunk::data::{ChunkData, RawChunkData};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use fnv::{FnvBuildHasher, FnvHashMap};
use nalgebra::Vector3;
use rc_networking::constants::CHUNK_SIZE;
use std::collections::HashMap;

pub mod builder;
pub mod data;
pub mod lookup;
pub mod mesh;
pub mod nearby_cache;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkSystem::new())
            .add_system(mesh_builder)
            .add_event::<RerenderChunkFlag>();
    }
}

#[derive(Resource)]
pub struct ChunkSystem {
    pub chunks: HashMap<Vector3<i32>, ChunkData, FnvBuildHasher>,
}

impl ChunkSystem {
    pub fn new() -> ChunkSystem {
        ChunkSystem {
            chunks: FnvHashMap::default(),
        }
    }

    /// Creates a new chunk from data
    pub fn create_chunk(
        &mut self,
        position: Vector3<i32>,
        data: RawChunkData,
        commands: &mut Commands,
        asset_service: &AssetService,
        rerender_chunk: &mut EventWriter<RerenderChunkFlag>,
    ) {
        let entity = commands
            .spawn(asset_service.texture_atlas_material.clone())
            .insert(Transform::from_translation(Vec3::new(
                (position.x * CHUNK_SIZE as i32) as f32,
                (position.y * CHUNK_SIZE as i32) as f32,
                (position.z * CHUNK_SIZE as i32) as f32,
            )))
            .insert(GlobalTransform::default())
            .insert(Visibility::default())
            .insert(ComputedVisibility::default())
            //TODO: Remove once bevy has fixed its shitty AABB generation
            .insert(Aabb::from_min_max(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(16.0, 16.0, 16.0),
            ))
            .id();

        let chunk = ChunkData::new(data, entity, position);

        self.chunks.insert(position, chunk);

        rerender_chunk.send(RerenderChunkFlag {
            chunk: position,
            context: RerenderChunkFlagContext::Surrounding,
        });
    }
}
