//
// Handles chunk loading, chunk unloading and chunk animations
//

use crate::services::settings_service::SettingsService;
use crate::services::ServicesContext;
use wgpu::{BindGroupLayout, Device};
use crate::world::generator::World;
use crate::services::chunk_service::chunk::{Chunk, ChunkData};
use crate::block::Block;

pub mod mesh;
pub mod chunk;

pub struct ChunkService {
    pub(crate) bind_group_layout: BindGroupLayout,
    pub(crate) chunks: Vec<Chunk>,
    pub(crate) vertices: u64
}

impl ChunkService {

    pub fn new(settings: &SettingsService, context: &mut ServicesContext) -> ChunkService {

        // Create the chunk bind group layout
        let bind_group_layout = context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: true
                    },
                }
            ]
        });

        let mut service = ChunkService {
            bind_group_layout,
            chunks: Vec::new(),
            vertices: 0
        };

        //TODO: Remove this once we have networking
        for x in -(settings.render_distance as i32)..(settings.render_distance as i32) {
            for z in -(settings.render_distance as i32)..(settings.render_distance as i32) {
                let data = ChunkService::generate_chunk(x, z, context.blocks);
                service.load_chunk(context.device, data, [x, z], &settings);
            }
        }

        service
    }

    //TODO: Remove this once we have networking setup
    fn generate_chunk(x: i32, z: i32, blocks: &Vec<Block>) -> ChunkData {
        return World::generate_chunk(x, z, blocks);
    }

    pub fn load_chunk(&mut self, device: &Device, data: ChunkData, chunk_coords: [i32; 2], settings: &SettingsService) {
        let mut chunk = Chunk::new(data, chunk_coords);

        chunk.generate_mesh(settings);
        self.vertices += chunk.vertices.as_ref().unwrap().len() as u64;
        chunk.create_buffers(device, &self.bind_group_layout);

        self.chunks.push(chunk)
    }
}