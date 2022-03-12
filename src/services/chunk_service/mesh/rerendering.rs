use crate::render::camera::Camera;
use crate::render::device::get_device;
use crate::render::RenderState;
use crate::services::chunk_service::chunk::{ChunkData, Chunks};
use crate::services::chunk_service::lighting::UpdateChunkLighting;
use crate::services::chunk_service::mesh::culling::ViewableDirection;
use crate::services::chunk_service::mesh::MeshData;
use crate::services::chunk_service::ChunkService;
use crate::services::settings_service::{SettingsService, CHUNK_SIZE};
use nalgebra::Matrix4;
use nalgebra::Vector3;
use rayon::iter::IntoParallelRefIterator;
use specs::prelude::ParallelIterator;
use specs::{Component, DenseVecStorage, Entities, Entity, Join, Read, ReadStorage, Write};
use specs::{System, WriteStorage};
use std::time::SystemTime;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BindGroup, BindGroupLayout, BindingResource, BufferBinding};

// TODO: Prioritise rendering visible chunks first

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

pub const LOW_MEMORY_WARNING_PERIOD: f32 = 5.0;
pub const LOW_MEMORY_MINIMUM_KB: u64 = 250000;

pub struct UpdateChunkMesh {
    pub chunk: Vector3<i32>,
    pub opaque_model: MeshData,
    pub translucent_model: MeshData,
    pub viewable_map: Option<[[[ViewableDirection; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
    pub model_bind_group: Option<BindGroup>,
}

pub struct UpdateChunkGraphics {
    pub mesh: UpdateChunkMesh,
    pub lighting: UpdateChunkLighting,
}

impl Component for UpdateChunkGraphics {
    type Storage = DenseVecStorage<Self>;
}
impl Default for UpdateChunkGraphics {
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
        Write<'a, ChunkService>,
        Entities<'a>,
        WriteStorage<'a, UpdateChunkGraphics>,
        Read<'a, Camera>,
    );

    fn run(
        &mut self,
        (
            mut flags,
            chunks,
            settings,
            mut chunk_service,
            entities,
            mut update_graphics,
            camera,
        ): Self::SystemData,
    ) {
        // Create indexed by location chunks array
        let chunks_loc = Chunks::new(chunks.join().collect::<Vec<&ChunkData>>());

        // Poll system resources
        chunk_service.system.poll();

        // If the system has less than minimum amount of ram refuse to build mesh
        if !chunk_service.system.should_alloc(LOW_MEMORY_MINIMUM_KB) {
            // Only log message every 5 seconds
            if chunk_service
                .low_memory_reminded
                .elapsed()
                .unwrap()
                .as_secs_f32()
                > LOW_MEMORY_WARNING_PERIOD
            {
                chunk_service.system.memory_warn();

                chunk_service.low_memory_reminded = SystemTime::now();
            }
            return;
        }

        let chunks_to_compute = get_closest_chunks(&entities, &flags, camera.eye.coords);

        let meshes = chunks_to_compute
            .par_iter()
            .map(|(to_delete, pos)| {
                // If the chunk exists
                if let Option::Some(chunk) = chunks_loc.get_loc(pos.chunk) {
                    assert_eq!(chunk.position, pos.chunk);

                    // Generate mesh & gpu buffers
                    let mut update = chunk.generate_mesh(&chunks_loc, &settings);

                    update.create_buffers(&chunk_service.model_bind_group_layout);

                    let lighting_update = chunk.calculate_lighting(&chunks_loc);

                    // Output result into lock
                    (Some(update), Some(lighting_update), *to_delete)
                } else {
                    (None, None, *to_delete)
                }
            })
            .collect::<Vec<(Option<UpdateChunkMesh>, Option<UpdateChunkLighting>, Entity)>>();

        // Apply the results of the work
        for (mesh, lighting, to_delete) in meshes {
            if let Option::Some(mesh) = mesh {
                if let Option::Some(lighting) = lighting {
                    entities
                        .build_entity()
                        .with(UpdateChunkGraphics { mesh, lighting }, &mut update_graphics)
                        .build();
                }
            }
            flags.remove(to_delete);
        }
    }
}

fn get_closest_chunks<'a>(
    entities: &Entities,
    flags: &'a WriteStorage<RerenderChunkFlag>,
    camera: Vector3<f32>,
) -> Vec<(Entity, &'a RerenderChunkFlag)> {
    let chunk_size = CHUNK_SIZE as i32;

    let mut chunks = (entities, flags)
        .join()
        .collect::<Vec<(Entity, &RerenderChunkFlag)>>();

    chunks.sort_unstable_by(|(_, flag_1), (_, flag_2)| {
        let chunk_center_1 = Vector3::new(
            (flag_1.chunk.x * chunk_size) as f32 + (chunk_size / 2) as f32,
            (flag_1.chunk.y * chunk_size) as f32 + (chunk_size / 2) as f32,
            (flag_1.chunk.z * chunk_size) as f32 + (chunk_size / 2) as f32,
        );

        let chunk_center_2 = Vector3::new(
            (flag_2.chunk.x * chunk_size) as f32 + (chunk_size / 2) as f32,
            (flag_2.chunk.y * chunk_size) as f32 + (chunk_size / 2) as f32,
            (flag_2.chunk.z * chunk_size) as f32 + (chunk_size / 2) as f32,
        );

        let offset_1 = chunk_center_1 - camera;
        let distance_1: f32 = offset_1.x.abs() + offset_1.y.abs() + offset_1.z.abs();

        let offset_2 = chunk_center_2 - camera;
        let distance_2: f32 = offset_2.x.abs() + offset_2.y.abs() + offset_2.z.abs();

        distance_1.partial_cmp(&distance_2).unwrap()
    });

    // Only render 80 chunks at a time
    chunks.truncate(80);

    chunks
}

impl UpdateChunkMesh {
    pub fn create_buffers(&mut self, bind_group_layout: &BindGroupLayout) {
        let opaque_vertices = &self.opaque_model.vertices;
        let translucent_vertices = &self.translucent_model.vertices;

        if opaque_vertices.len() != 0 {
            let vertex_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
                label: Some("Chunk Opaque Mesh Data Buffer"),
                contents: &bytemuck::cast_slice(&opaque_vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            self.opaque_model.vertices_buffer = Some(vertex_buffer);
        }

        if translucent_vertices.len() != 0 {
            let vertex_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
                label: Some("Chunk Translucent Mesh Data Buffer"),
                contents: &bytemuck::cast_slice(&translucent_vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            self.translucent_model.vertices_buffer = Some(vertex_buffer);
        }

        let opaque_indices = &self.opaque_model.indices;
        let translucent_indices = &self.translucent_model.indices;
        self.opaque_model.indices_buffer_len = opaque_indices.len() as u32;
        self.translucent_model.indices_buffer_len = translucent_indices.len() as u32;

        if self.opaque_model.indices_buffer_len != 0 {
            let indices_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
                label: Some("Chunk Opaque Mesh Data Indices Buffer"),
                contents: &bytemuck::cast_slice(&opaque_indices),
                usage: wgpu::BufferUsages::INDEX,
            });
            self.opaque_model.indices_buffer = Some(indices_buffer);
        }

        if self.translucent_model.indices_buffer_len != 0 {
            let indices_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
                label: Some("Chunk Translucent Mesh Data Indices Buffer"),
                contents: &bytemuck::cast_slice(&translucent_indices),
                usage: wgpu::BufferUsages::INDEX,
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

        let model_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
            label: Some("Chunk translation matrix buffer"),
            contents: &bytemuck::cast_slice(&[model.clone()]),
            usage: wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        let model_bind_group = get_device().create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Chunk translation matrix bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &model_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        self.model_bind_group = Some(model_bind_group);
    }
}
