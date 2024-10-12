use crate::systems::asset::AssetService;
use crate::systems::chunk::builder::{ATTRIBUTE_LIGHTING_COLOR, ATTRIBUTE_SKYLIGHT_STRENGTH, ATTRIBUTE_WIND_STRENGTH, ChunkRebuiltEvent, mesh_scheduler, mesh_updater, RerenderChunkRequest};
use crate::systems::chunk::data::ChunkData;
use crate::systems::chunk::request::request_chunks;
use bevy::prelude::*;
use bevy::render::mesh::{PrimitiveTopology, VertexAttributeValues};
use bevy::render::primitives::Aabb;
use bevy::render::render_asset::RenderAssetUsages;
use fnv::{FnvBuildHasher, FnvHashMap};
use nalgebra::{Vector2, Vector3};
use rc_shared::chunk::{ChunkDataStorage, ChunkSystemTrait};
use rc_shared::CHUNK_SIZE;
use std::collections::HashMap;
use rc_shared::chunk_column::ChunkColumnData;
use crate::state::AppState;
use crate::systems::asset::parsing::message_pack::MessagePackAssetLoader;
use crate::systems::chunk::builder::builder::setup_mesh_builder_context;
use crate::systems::chunk::column::receive_column_updates;
use crate::systems::chunk::flags::ChunkFlagsBitMap;
use crate::systems::chunk::static_world_data::{save_surroundings_system, StaticWorldData};

pub mod builder;
pub mod data;
pub mod lookup;
pub mod nearby_cache;
mod request;
pub mod static_world_data;
mod nearby_chunk_map;
pub mod flags;
mod edge;
mod condensed_spacial_data;
mod column;
mod nearby_column_cache;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkSystem::new())
            .add_systems(OnExit(AppState::Loading), setup_mesh_builder_context)
            .add_systems(Update, (mesh_scheduler, mesh_updater).run_if(in_state(AppState::MainMenu)))
            .add_systems(Update, (mesh_scheduler, mesh_updater).run_if(in_state(AppState::Connecting)))
            .add_systems(Update, (mesh_scheduler, mesh_updater).run_if(in_state(AppState::InGame)))
            .add_event::<RerenderChunkRequest>()
            .add_event::<ChunkRebuiltEvent>()
            .add_systems(Update, request_chunks)
            // Static world data
            .init_asset::<StaticWorldData>()
            .init_asset_loader::<MessagePackAssetLoader<StaticWorldData>>()
            .add_systems(Update, save_surroundings_system)
            .add_systems(Update, receive_column_updates);
    }
}

#[derive(Resource)]
pub struct ChunkSystem {
    pub chunks: HashMap<Vector3<i32>, ChunkData, FnvBuildHasher>,
    pub chunk_columns: HashMap<Vector2<i32>, ChunkColumnData, FnvBuildHasher>,

    /// A list of all chunks that have rerender requests outstanding
    pub requested_chunks: Vec<Vector3<i32>>
}

impl ChunkSystem {
    pub fn new() -> ChunkSystem {
        ChunkSystem {
            chunks: FnvHashMap::default(),
            chunk_columns: FnvHashMap::default(),
            requested_chunks: vec![]
        }
    }

    /// Creates a new chunk from data
    pub fn create_chunk(
        &mut self,
        position: Vector3<i32>,
        data: ChunkDataStorage,
        commands: &mut Commands,
        asset_service: &AssetService,
        meshes: &mut Assets<Mesh>,
    ) {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![] as Vec<[f32; 3]>);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![] as Vec<[f32; 3]>);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![] as Vec<[f32; 2]>);
        mesh.insert_attribute(ATTRIBUTE_LIGHTING_COLOR, VertexAttributeValues::Float32x4(vec![]));
        mesh.insert_attribute(ATTRIBUTE_SKYLIGHT_STRENGTH, VertexAttributeValues::Uint8x4(vec![]));

        let opaque = meshes.add(mesh.clone());

        mesh.insert_attribute(ATTRIBUTE_WIND_STRENGTH, VertexAttributeValues::Float32(vec![]));

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
            .insert(VisibilityBundle::default())
            .with_children(|c| {
                opaque_entity = Some(
                    c.spawn(asset_service.opaque_texture_atlas_material.clone())
                        .insert(Transform::default())
                        .insert(GlobalTransform::default())
                        .insert(VisibilityBundle::default())
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
                        .insert(VisibilityBundle::default())
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
            position,
            entity,
            opaque_entity.unwrap(),
            transparent_entity.unwrap(),
            opaque,
            translucent,
        );

        self.chunks.insert(position, chunk);

        // Recompute onedge for all surrounding chunks
        for x in (position.x - 1)..=(position.x + 1) {
            for y in (position.y - 1)..=(position.y + 1) {
                for z in (position.z - 1)..=(position.z + 1) {
                    self.recompute_on_edge(Vector3::new(x, y, z));
                }
            }
        }
    }

    pub fn unload_all_chunks(&mut self, commands: &mut Commands) {
        for (_, chunk) in self.chunks.drain() {
            if let Some(handles) = chunk.handles {
                commands.entity(handles.entity).despawn_recursive();
            }
        }
    }

    pub fn unload_chunk(&mut self, position: Vector3<i32>, commands: &mut Commands) {
        if let Some(chunk) = self.chunks.remove(&position) {
            if let Some(handles) = chunk.handles {
                commands.entity(handles.entity).despawn_recursive();
            }
        }

        // All surrounding chunks are now on edge
        for x in (position.x - 1)..=(position.x + 1) {
            for y in (position.y - 1)..=(position.y + 1) {
                for z in (position.z - 1)..=(position.z + 1) {
                    if let Some(data) = self.chunks.get_mut(&Vector3::new(x, y, z)) {
                        data.flags.add_flag(ChunkFlagsBitMap::AtEdge);
                    }
                }
            }
        }
    }
}

impl ChunkSystemTrait for ChunkSystem {
    fn get_raw_chunk(&self, pos: &Vector3<i32>) -> Option<&ChunkDataStorage> {
        self.chunks.get(pos).map(|v| &v.world)
    }
    fn get_raw_chunk_mut(&mut self, pos: &Vector3<i32>) -> Option<&mut ChunkDataStorage> {
        self.chunks.get_mut(pos).map(|v| &mut v.world)
    }
}
