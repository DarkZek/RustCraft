use crate::systems::asset::AssetService;
use crate::systems::chunk::builder::{
    mesh_builder, RerenderChunkFlag, RerenderChunkFlagContext, ATTRIBUTE_LIGHTING_COLOR,
};
use crate::systems::chunk::data::{ChunkData, RawChunkData};
use crate::systems::chunk::request::request_chunks;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::primitives::Aabb;
use fnv::{FnvBuildHasher, FnvHashMap};
use nalgebra::Vector3;
use rc_networking::constants::CHUNK_SIZE;
use std::collections::HashMap;

pub mod biome;
pub mod builder;
pub mod data;
pub mod lookup;
pub mod mesh;
pub mod nearby_cache;
mod request;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkSystem::new())
            .add_systems(Update, mesh_builder)
            .add_event::<RerenderChunkFlag>()
            .add_systems(Update, request_chunks);
    }
}

#[derive(Resource)]
pub struct ChunkSystem {
    pub chunks: HashMap<Vector3<i32>, ChunkData, FnvBuildHasher>,
    pub requested_chunks: Vec<Vector3<i32>>,
}

impl ChunkSystem {
    pub fn new() -> ChunkSystem {
        ChunkSystem {
            chunks: FnvHashMap::default(),
            requested_chunks: vec![],
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
        meshes: &mut Assets<Mesh>,
    ) {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![] as Vec<[f32; 3]>);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![] as Vec<[f32; 3]>);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![] as Vec<[f32; 2]>);
        mesh.insert_attribute(ATTRIBUTE_LIGHTING_COLOR, vec![] as Vec<[f32; 4]>);

        let opaque = meshes.add(mesh.clone());
        let translucent = meshes.add(mesh);

        let mut opaque_entity = None;
        let mut transparent_entity = None;
        let entity = commands
            .spawn(Transform::from_translation(Vec3::new(
                (position.x * CHUNK_SIZE as i32) as f32,
                (position.y * CHUNK_SIZE as i32) as f32,
                (position.z * CHUNK_SIZE as i32) as f32,
            )))
            .insert(GlobalTransform::default())
            .insert(Visibility::default())
            .insert(ComputedVisibility::default())
            .with_children(|c| {
                opaque_entity = Some(
                    c.spawn(asset_service.opaque_texture_atlas_material.clone())
                        .insert(Transform::default())
                        .insert(GlobalTransform::default())
                        .insert(Visibility::default())
                        .insert(ComputedVisibility::default())
                        .insert(Aabb::from_min_max(
                            Vec3::new(0.0, 0.0, 0.0),
                            Vec3::new(16.0, 16.0, 16.0),
                        ))
                        .insert(opaque.clone())
                        .id(),
                );
                transparent_entity = Some(
                    c.spawn(asset_service.translucent_texture_atlas_material.clone())
                        .insert(Transform::default())
                        .insert(GlobalTransform::default())
                        .insert(Visibility::default())
                        .insert(ComputedVisibility::default())
                        .insert(Aabb::from_min_max(
                            Vec3::new(0.0, 0.0, 0.0),
                            Vec3::new(16.0, 16.0, 16.0),
                        ))
                        .insert(translucent.clone())
                        .id(),
                );
            })
            .id();

        let chunk = ChunkData::new(
            data,
            entity,
            opaque_entity.unwrap(),
            transparent_entity.unwrap(),
            position,
            opaque,
            translucent,
        );

        self.chunks.insert(position, chunk);

        rerender_chunk.send(RerenderChunkFlag {
            chunk: position,
            context: RerenderChunkFlagContext::Surrounding,
        });
    }
}
