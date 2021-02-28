use crate::helpers::chunk_by_loc_from_write;
use crate::render::RenderState;
use crate::services::chunk_service::chunk::{ChunkData, Chunks};
use crate::services::chunk_service::mesh::culling::ViewableDirection;
use crate::services::chunk_service::mesh::MeshData;
use crate::services::chunk_service::ChunkService;
use crate::services::settings_service::{SettingsService, CHUNK_SIZE};
use nalgebra::Matrix4;
use nalgebra::Vector3;
use specs::{Component, DenseVecStorage, Entities, Join, Read, ReadStorage};
use specs::{System, WriteStorage};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BindGroup, BindGroupLayout, Device};

pub struct RerenderChunkFlag {
    pub chunk: Vector3<i32>,
}
impl Component for RerenderChunkFlag {
    type Storage = DenseVecStorage<Self>;
}
impl Default for RerenderChunkFlag {
    fn default() -> Self {
        unimplemented!()
    }
}

pub struct UpdateChunkMesh {
    pub chunk: Vector3<i32>,
    pub opaque_model: MeshData,
    pub translucent_model: MeshData,
    pub viewable_map: Option<[[[ViewableDirection; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
    pub model_bind_group: Option<BindGroup>,
}
impl Component for UpdateChunkMesh {
    type Storage = DenseVecStorage<Self>;
}
impl Default for UpdateChunkMesh {
    fn default() -> Self {
        unimplemented!()
    }
}

pub struct ChunkRerenderSystem;

impl<'a> System<'a> for ChunkRerenderSystem {
    type SystemData = (
        WriteStorage<'a, RerenderChunkFlag>,
        ReadStorage<'a, ChunkData>,
        Read<'a, SettingsService>,
        Read<'a, ChunkService>,
        Read<'a, RenderState>,
        Entities<'a>,
        WriteStorage<'a, UpdateChunkMesh>,
    );

    fn run(
        &mut self,
        (
            mut flags,
            mut chunks,
            settings,
            chunk_service,
            render_system,
            entities,
            mut update_meshes,
        ): Self::SystemData,
    ) {
        let chunks_loc = Chunks::new(chunks.join().collect::<Vec<&ChunkData>>());

        for flag in flags.join() {
            if let Option::Some(chunk) = chunks_loc.get_loc(flag.chunk) {
                let mut update = chunk.generate_mesh(&chunks_loc, &settings);
                update.create_buffers(
                    &render_system.device,
                    &chunk_service.model_bind_group_layout,
                );
                entities
                    .build_entity()
                    .with(update, &mut update_meshes)
                    .build();
            }
        }

        flags.clear();
    }
}

impl UpdateChunkMesh {
    pub fn create_buffers(&mut self, device: &Device, bind_group_layout: &BindGroupLayout) {
        let opaque_vertices = &self.opaque_model.vertices;
        let translucent_vertices = &self.translucent_model.vertices;

        if opaque_vertices.len() != 0 {
            let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: &bytemuck::cast_slice(&opaque_vertices),
                usage: wgpu::BufferUsage::VERTEX,
            });
            self.opaque_model.vertices_buffer = Some(vertex_buffer);
        }

        if translucent_vertices.len() != 0 {
            let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: &bytemuck::cast_slice(&translucent_vertices),
                usage: wgpu::BufferUsage::VERTEX,
            });
            self.translucent_model.vertices_buffer = Some(vertex_buffer);
        }

        let opaque_indices = &self.opaque_model.indices;
        let translucent_indices = &self.translucent_model.indices;
        self.opaque_model.indices_buffer_len = opaque_indices.len() as u32;
        self.translucent_model.indices_buffer_len = translucent_indices.len() as u32;

        if self.opaque_model.indices_buffer_len != 0 {
            let indices_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: &bytemuck::cast_slice(&opaque_indices),
                usage: wgpu::BufferUsage::INDEX,
            });
            self.opaque_model.indices_buffer = Some(indices_buffer);
        }

        if self.translucent_model.indices_buffer_len != 0 {
            let indices_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: &bytemuck::cast_slice(&translucent_indices),
                usage: wgpu::BufferUsage::INDEX,
            });
            self.translucent_model.indices_buffer = Some(indices_buffer);
        }

        // Create model buffer
        let model: [[f32; 4]; 4] = Matrix4::new_translation(&Vector3::new(
            self.chunk.x as f32 * 16.0,
            self.chunk.y as f32 * 16.0,
            self.chunk.z as f32 * 16.0,
        ))
        .into();

        let model_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: &bytemuck::cast_slice(&[model.clone()]),
            usage: wgpu::BufferUsage::UNIFORM
                | wgpu::BufferUsage::COPY_DST
                | wgpu::BufferUsage::COPY_SRC,
        });

        let model_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    model_buffer
                        .slice(0..std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress),
                ),
            }],
            label: None,
        });

        self.model_bind_group = Some(model_bind_group);
    }
}
