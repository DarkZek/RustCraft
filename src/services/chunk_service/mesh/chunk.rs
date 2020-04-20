use wgpu::{Device, BindGroupLayout};
use cgmath::{Matrix4, Vector3};
use crate::services::chunk_service::mesh::culling::{calculate_viewable, ViewableDirection};
use crate::services::chunk_service::chunk::Chunk;
use crate::services::settings_service::{CHUNK_SIZE, CHUNK_HEIGHT};

impl Chunk {

    pub fn create_buffers(&mut self, device: &Device, model_bind_group_layout: &BindGroupLayout) {
        let vertices = self.vertices.as_ref().unwrap();

        let vertex_buffer = device
            .create_buffer_mapped(vertices.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(vertices.as_slice());

        self.vertices_buffer = Some(vertex_buffer);

        let indices = self.indices.take().unwrap();
        self.indices_buffer_len = indices.len() as u32;

        let indices_buffer = device
            .create_buffer_mapped(indices.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(indices.as_slice());

        self.indices_buffer = Some(indices_buffer);

        // Create model buffer
        let model: [[f32; 4]; 4] = Matrix4::from_translation(Vector3 {
            x: self.x as f32 * 16.0,
            y: 0.0,
            z: self.z as f32 * 16.0
        }).into();

        let model_buffer = device
            .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&[(model)]);

        let model_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &model_bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &model_buffer,
                        range: 0..std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
                    }
                }
            ],
        });

        self.model_bind_group = Some(model_bind_group);
    }

    pub fn generate_viewable_map(&mut self) {

        let mut data = [[[ViewableDirection(0); CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE];
        let world = self.world;

        for x in 0..world.len() {
            for z in 0..world[0][0].len() {
                for y in 0..world[0].len() {
                    let viewable = calculate_viewable(&self, [x, y, z]);

                    data[x][y][z] = ViewableDirection(viewable);
                }
            }
        }

        self.viewable_map = Some(data);
    }
}