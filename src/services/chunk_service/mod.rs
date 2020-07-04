//
// Handles chunk loading, chunk unloading and chunk animations
//

use crate::block::Block;
use crate::render::camera::Camera;
use crate::services::chunk_service::chunk::{Chunk, ChunkData};
use crate::services::chunk_service::frustum_culling::calculate_frustum_culling;
use crate::services::settings_service::{SettingsService, CHUNK_SIZE};
use crate::services::ServicesContext;
use crate::world::generator::World;
use nalgebra::Vector3;
use std::collections::HashMap;
use wgpu::{BindGroupLayout, Device};

pub mod chunk;
pub mod frustum_culling;
pub mod mesh;

pub struct ChunkService {
    pub bind_group_layout: BindGroupLayout,
    pub chunks: HashMap<Vector3<i32>, Chunk>,
    pub viewable_chunks: Vec<Vector3<i32>>,
    pub visible_chunks: Vec<Vector3<i32>>,
    pub vertices_count: u64,
    pub chunk_keys: Vec<Vector3<i32>>,
}

impl ChunkService {
    pub fn new(settings: &SettingsService, context: &mut ServicesContext) -> ChunkService {
        let bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: true },
            }],
            label: None
        };

        // Create the chunk bind group layout
        let bind_group_layout = context
            .device
            .create_bind_group_layout(&bind_group_layout_descriptor);

        let mut service = ChunkService {
            bind_group_layout,
            chunks: HashMap::with_capacity(16 * CHUNK_SIZE * CHUNK_SIZE),
            viewable_chunks: vec![],
            visible_chunks: vec![],
            vertices_count: 0,
            chunk_keys: Vec::new(),
        };

        //TODO: Remove this once we have networking
        for x in -(settings.render_distance as i32)..(settings.render_distance as i32) {
            for z in -(settings.render_distance as i32)..(settings.render_distance as i32) {
                for y in 0..16 {
                    let data = ChunkService::generate_chunk(x, y, z, context.blocks);
                    service.load_chunk(context.device, data, Vector3::new(x, y, z), &settings);
                }
            }
        }

        for i in 0..service.chunks.len() {
            let chunk_key = service.chunk_keys.get(i).unwrap();

            let mesh_data = {
                let chunk = service.chunks.get(chunk_key).unwrap();

                chunk.generate_mesh(&service, settings)
            };

            // Add new vertices to count
            service.vertices_count += mesh_data.vertices.len() as u64;

            let chunk = service.chunks.get_mut(chunk_key).unwrap();

            chunk.update_mesh(mesh_data);
            chunk.create_buffers(&context.device, &service.bind_group_layout);
        }

        service
    }

    //TODO: Remove this once we have networking setup
    fn generate_chunk(x: i32, y: i32, z: i32, blocks: &Vec<Block>) -> Option<ChunkData> {
        return World::generate_chunk(Vector3::new(x, y, z), blocks);
    }

    pub fn load_chunk(
        &mut self,
        _device: &Device,
        data: Option<ChunkData>,
        chunk_coords: Vector3<i32>,
        _settings: &SettingsService,
    ) {
        let chunk = if data.is_some() {
            Chunk::new(data.unwrap(), chunk_coords)
        } else {
            Chunk::new_empty(chunk_coords)
        };

        // chunk.generate_mesh(&self);
        // self.vertices += chunk.vertices.as_ref().unwrap().len() as u64;
        // chunk.create_buffers(device, &self.bind_group_layout);

        if chunk.world.is_some() {
            self.viewable_chunks.push(chunk_coords);
        }

        self.chunk_keys.push(chunk_coords.clone());
        self.chunks.insert(chunk_coords.clone(), chunk);
    }

    pub fn update_frustum_culling(&mut self, camera: &Camera) {
        self.visible_chunks =
            calculate_frustum_culling(camera, &self.viewable_chunks, &self.chunks);
    }
}

impl Default for ChunkService {
    fn default() -> Self {
        unimplemented!()
    }
}