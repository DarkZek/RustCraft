//
// Handles chunk loading, chunk unloading and chunk animations
//

use crate::block::Block;
use crate::render::camera::Camera;
use crate::services::chunk_service::chunk::{Chunk, ChunkBlockData, ChunkData, Chunks};
use crate::services::chunk_service::frustum_culling::calculate_frustum_culling;
use crate::services::settings_service::{SettingsService, CHUNK_SIZE};
use crate::services::ServicesContext;
use nalgebra::Vector3;
use specs::{World, WorldExt};
use std::collections::HashMap;

use crate::block::blocks::BlockStates;
use std::sync::Arc;
use wgpu::BindGroupLayout;

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

    //Temp
    previous_player_rot: f32,
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
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer {
                    dynamic: true,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
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
            previous_player_rot: 0.0,
        };

        let chunks = Chunks(HashMap::with_capacity(16 * CHUNK_SIZE * CHUNK_SIZE));

        universe.insert(chunks);

        service
    }

    pub fn load_chunk(
        &mut self,
        data: Option<ChunkBlockData>,
        chunk_coords: Vector3<i32>,
        chunks: &mut Chunks,
    ) {
        if data.is_some() {
            let mut chunk = ChunkData::new(data.unwrap(), chunk_coords);

            self.viewable_chunks.push(chunk_coords);
            if chunk.indices_buffer.is_some() {
                self.visible_chunks.push(chunk_coords);
            }

            self.chunk_keys.push(chunk_coords.clone());
            chunks
                .0
                .insert(chunk_coords.clone(), Chunk::Tangible(chunk));
        } else {
            self.chunk_keys.push(chunk_coords.clone());
            chunks.0.insert(chunk_coords.clone(), Chunk::Intangible);
        }
    }

    pub fn update_frustum_culling(&mut self, camera: &Camera, chunks: &Chunks) {
        // To 3 dp
        if (camera.yaw * 100.0).round() == self.previous_player_rot {
            return;
        }

        self.previous_player_rot = (camera.yaw * 100.0).round();

        self.visible_chunks = calculate_frustum_culling(camera, &self.viewable_chunks, &chunks.0);
    }
}

impl Default for ChunkService {
    fn default() -> Self {
        unimplemented!()
    }
}
