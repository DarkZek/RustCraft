use crate::services::asset::AssetService;
use crate::services::chunk::data::{ChunkData, RawChunkData};
use crate::services::chunk::systems::mesh_builder::{mesh_builder, RerenderChunkFlag};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use fnv::{FnvBuildHasher, FnvHashMap};
use nalgebra::Vector3;
use rc_protocol::constants::CHUNK_SIZE;
use rc_protocol::protocol::clientbound::chunk_update::FullChunkUpdate;
use std::collections::HashMap;

pub mod data;
pub mod lookup;
pub mod systems;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkService::new())
            .add_system(mesh_builder);
    }
}

#[derive(Resource)]
pub struct ChunkService {
    pub chunks: HashMap<Vector3<i32>, ChunkData, FnvBuildHasher>,
}

impl ChunkService {
    pub fn new() -> ChunkService {
        ChunkService {
            chunks: FnvHashMap::default(),
        }
    }

    /// Loads a chunk from network into the textures by creating an entity and ChunkData entry
    pub fn load_chunk(
        &mut self,
        position: Vector3<i32>,
        data: &FullChunkUpdate,
        commands: &mut Commands,
        asset_service: &AssetService,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) {
        let entity = commands
            .spawn(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 0.0 })),
                material: asset_service.texture_atlas_material.clone(),
                transform: Transform::from_translation(Vec3::new(
                    (position.x * CHUNK_SIZE as i32) as f32,
                    (position.y * CHUNK_SIZE as i32) as f32,
                    (position.z * CHUNK_SIZE as i32) as f32,
                )),
                ..default()
            })
            .insert(RerenderChunkFlag { chunk: position })
            //TODO: Remove once bevy has fixed its shitty AABB generation
            .insert(Aabb::from_min_max(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(16.0, 16.0, 16.0),
            ))
            .id();

        self.chunks
            .insert(position, ChunkData::new(data.data, entity, position));
    }

    /// Creates a new chunk from data
    pub fn create_chunk(
        &mut self,
        position: Vector3<i32>,
        data: RawChunkData,
        commands: &mut Commands,
        asset_service: &AssetService,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) {
        let entity = commands
            .spawn(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 0.0 })),
                material: asset_service.texture_atlas_material.clone(),
                transform: Transform::from_translation(Vec3::new(
                    (position.x * CHUNK_SIZE as i32) as f32,
                    (position.y * CHUNK_SIZE as i32) as f32,
                    (position.z * CHUNK_SIZE as i32) as f32,
                )),
                ..default()
            })
            .insert(RerenderChunkFlag { chunk: position })
            //TODO: Remove once bevy has fixed its shitty AABB generation
            .insert(Aabb::from_min_max(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(16.0, 16.0, 16.0),
            ))
            .id();

        let chunk = ChunkData::new(data, entity, position);

        self.chunks.insert(position, chunk);
    }
}
