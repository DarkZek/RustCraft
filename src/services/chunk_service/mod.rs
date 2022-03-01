//
// Handles chunk loading, chunk unloading and chunk animations
//

use crate::render::camera::Camera;
use crate::services::chunk_service::chunk::{ChunkData, ChunkEntityLookup, RawChunkData};
use crate::services::chunk_service::frustum_culling::calculate_frustum_culling;
use crate::services::settings_service::SettingsService;
use crate::services::ServicesContext;
use nalgebra::Vector3;
use specs::{Entities, ReadStorage, World, Write, WriteStorage};

use crate::game::resources::SystemResources;
use crate::services::chunk_service::mesh::rerendering::RerenderChunkFlag;
use std::cmp::Ordering;
use std::time::SystemTime;
use wgpu::{BindGroupLayout, BufferBindingType};

pub mod blocks;
pub mod chunk;
pub mod frustum_culling;
pub mod lighting;
pub mod mesh;

pub struct ChunkService {
    pub model_bind_group_layout: BindGroupLayout,
    pub viewable_chunks: Vec<Vector3<i32>>,
    pub visible_chunks: Vec<Vector3<i32>>,
    pub vertices_count: u64,
    pub chunk_keys: Vec<Vector3<i32>>,

    //Pos when we last culled
    previous_player_yaw: f32,
    previous_player_pitch: f32,
    previous_player_pos: Vector3<f32>,

    system: SystemResources,
    low_memory_reminded: SystemTime,

    // Chunks have updated so update culling next frame
    update_culling: bool,
}

impl ChunkService {
    pub fn new(
        _settings: &SettingsService,
        context: &mut ServicesContext,
        universe: &mut World,
    ) -> ChunkService {
        let model_bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            }],
            label: Some("Chunk Bind Group"),
        };

        // Create the chunk bind group layout
        let model_bind_group_layout = context
            .device
            .create_bind_group_layout(&model_bind_group_layout_descriptor);

        let service = ChunkService {
            model_bind_group_layout,
            viewable_chunks: vec![],
            visible_chunks: vec![],
            vertices_count: 0,
            chunk_keys: Vec::new(),
            previous_player_yaw: 0.0,
            previous_player_pitch: 0.0,
            previous_player_pos: Vector3::zeros(),
            system: SystemResources::new(),
            low_memory_reminded: SystemTime::now(),
            update_culling: false,
        };

        service
    }

    pub fn load_chunk(
        &mut self,
        data: Option<RawChunkData>,
        chunk_coords: Vector3<i32>,
        entities: &mut Entities,
        chunks: &mut WriteStorage<ChunkData>,
        rerendering_chunks: &mut WriteStorage<RerenderChunkFlag>,
        chunk_entity_lookup: &mut Write<ChunkEntityLookup>,
    ) {
        if data.is_some() {
            let chunk = ChunkData::new(data.unwrap(), chunk_coords);

            self.viewable_chunks.push(chunk_coords);

            self.chunk_keys.push(chunk_coords.clone());

            let entity = entities.create();
            if let Err(err) = chunks.insert(entity.clone(), chunk) {
                log_error!(format!(
                    "Error creating entity for chunk {}: {}",
                    chunk_coords, err
                ));
            }

            chunk_entity_lookup.map.insert(chunk_coords, entity);

            // Flag the adjacent chunks for rerendering
            let rerender = [
                Vector3::new(chunk_coords.x + 1, chunk_coords.y, chunk_coords.z),
                Vector3::new(chunk_coords.x, chunk_coords.y + 1, chunk_coords.z),
                Vector3::new(chunk_coords.x, chunk_coords.y, chunk_coords.z + 1),
                Vector3::new(chunk_coords.x - 1, chunk_coords.y, chunk_coords.z),
                Vector3::new(chunk_coords.x, chunk_coords.y - 1, chunk_coords.z),
                Vector3::new(chunk_coords.x, chunk_coords.y, chunk_coords.z - 1),
            ];

            for rerender_pos in rerender.iter() {
                entities
                    .build_entity()
                    .with(
                        RerenderChunkFlag {
                            chunk: *rerender_pos,
                        },
                        rerendering_chunks,
                    )
                    .build();
            }

            self.sort_chunks();
        }
    }

    pub fn update_frustum_culling(&mut self, camera: &Camera, chunks: &ReadStorage<ChunkData>) {
        // To 3 dp
        if ((camera.yaw * 100.0).round() == self.previous_player_yaw
            && (camera.pitch * 100.0).round() == self.previous_player_pitch)
            && !self.update_culling
            && self.previous_player_pos.metric_distance(&camera.eye.coords) < 1.0
        {
            return;
        }

        self.update_culling = false;
        self.previous_player_pos = Vector3::new(camera.eye.x, camera.eye.y, camera.eye.z);

        self.previous_player_yaw = (camera.yaw * 100.0).round();
        self.previous_player_pitch = (camera.pitch * 100.0).round();

        self.visible_chunks = calculate_frustum_culling(camera, &self.viewable_chunks, &chunks);
    }

    // TODO: Run this every time the player moves between chunks
    pub fn sort_chunks(&mut self) {
        let player_pos = Vector3::new(0, 0, 0);

        self.chunk_keys.sort_by(|a, b| {
            if dist(&player_pos, a) > dist(&player_pos, b) {
                Ordering::Less
            } else if dist(&player_pos, a) < dist(&player_pos, b) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        self.visible_chunks.sort_by(|a, b| {
            if dist(&player_pos, a) > dist(&player_pos, b) {
                Ordering::Less
            } else if dist(&player_pos, a) < dist(&player_pos, b) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
    }
}

fn dist(v1: &Vector3<i32>, v2: &Vector3<i32>) -> u32 {
    ((v1.x - v2.x).abs() + (v1.y - v2.y).abs() + (v1.z - v2.z).abs()) as u32
}

impl Default for ChunkService {
    fn default() -> Self {
        unimplemented!()
    }
}
