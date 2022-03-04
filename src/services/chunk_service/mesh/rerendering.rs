use crate::render::camera::Camera;
use crate::render::RenderState;
use crate::services::chunk_service::chunk::{ChunkData, Chunks};
use crate::services::chunk_service::lighting::UpdateChunkLighting;
use crate::services::chunk_service::mesh::culling::ViewableDirection;
use crate::services::chunk_service::mesh::MeshData;
use crate::services::chunk_service::ChunkService;
use crate::services::settings_service::{SettingsService, CHUNK_SIZE};
use futures::executor::block_on;
use nalgebra::Matrix4;
use nalgebra::Vector3;
use specs::{Component, DenseVecStorage, Entities, Entity, Join, Read, ReadStorage, Write};
use specs::{System, WriteStorage};
use std::mem;
use std::mem::transmute;
use std::sync::Mutex;
use std::time::{Instant, SystemTime};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BindGroup, BindGroupLayout, BindingResource, BufferBinding, Device};

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
    pub(crate) mesh: UpdateChunkMesh,
    pub(crate) lighting: UpdateChunkLighting,
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
        Read<'a, RenderState>,
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
            render_system,
            entities,
            mut update_graphics,
            camera,
        ): Self::SystemData,
    ) {
        // Create indexed by location chunks array
        let chunks_loc = Chunks::new(chunks.join().collect::<Vec<&ChunkData>>());

        // Create list for parallel work to put result in to
        let meshes = Mutex::new(Vec::new());

        let mut processed_chunks = Vec::new();

        loop {
            // Poll system resources
            chunk_service.system.poll();

            let res =
                get_closest_chunk(&entities, &flags, camera.eye.coords, &mut processed_chunks);

            // No more chunks to process!
            if res.is_none() {
                break;
            }

            let (flag_entity, pos) = res.unwrap();

            // If the chunk exists
            if let Option::Some(chunk) = chunks_loc.get_loc(pos) {
                assert_eq!(chunk.position, pos);

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

                // Mark the thread as static so tokio doesn't complain, the thread ends by the time this function ends so is guaranteed to be valid memory.
                // let chunk = unsafe { mem::transmute::<&ChunkData, &'static ChunkData>(&chunk) };
                //
                // let t = tokio::spawn(async {
                // Generate mesh & gpu buffers
                let mut update = chunk.generate_mesh(&chunks_loc, &settings);

                update.create_buffers(
                    &render_system.device,
                    &chunk_service.model_bind_group_layout,
                );

                let lighting_update = chunk.calculate_lighting(&chunks_loc);

                // Output result into lock
                meshes
                    .lock()
                    .unwrap()
                    .push((Some(update), Some(lighting_update), flag_entity));
                // });
                //
                // block_on(t);
            } else {
                meshes.lock().unwrap().push((None, None, flag_entity));
            }

            // Limit chunks loaded per frame, just over a column
            if processed_chunks.len() == 24 {
                break;
            }
        }

        // Apply the results of the work
        for (mesh, lighting, to_delete) in meshes.into_inner().unwrap() {
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

fn get_closest_chunk(
    entities: &Entities,
    flags: &WriteStorage<RerenderChunkFlag>,
    camera: Vector3<f32>,
    processed: &mut Vec<Vector3<i32>>,
) -> Option<(Entity, Vector3<i32>)> {
    let mut closest = None;
    let mut entity = None;
    let mut closest_chunk_distance = 9999999.0;
    println!(
        "{}",
        (entities, flags)
            .join()
            .collect::<Vec<(Entity, &RerenderChunkFlag)>>()
            .len()
    );
    for (flag_entity, flag) in (entities, flags).join() {
        if processed.contains(&flag.chunk) {
            continue;
        }

        let chunk_center = Vector3::new(
            (flag.chunk.x * CHUNK_SIZE as i32) as f32 + (CHUNK_SIZE as f32 / 2.0),
            (flag.chunk.y * CHUNK_SIZE as i32) as f32 + (CHUNK_SIZE as f32 / 2.0),
            (flag.chunk.z * CHUNK_SIZE as i32) as f32 + (CHUNK_SIZE as f32 / 2.0),
        );

        let offset = chunk_center - camera;
        let distance: f32 = offset.x.abs() + offset.y.abs() + offset.z.abs();

        if distance < closest_chunk_distance {
            closest_chunk_distance = distance;
            entity = Some(flag_entity);
            closest = Some(flag);
        }
    }

    if closest.is_none() {
        return None;
    }

    processed.push(closest.as_ref().unwrap().chunk);

    return Some((entity.unwrap(), closest.as_ref().unwrap().chunk));
}

impl UpdateChunkMesh {
    pub fn create_buffers(&mut self, device: &Device, bind_group_layout: &BindGroupLayout) {
        let opaque_vertices = &self.opaque_model.vertices;
        let translucent_vertices = &self.translucent_model.vertices;

        if opaque_vertices.len() != 0 {
            let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: Some("Chunk Opaque Mesh Data Buffer"),
                contents: &bytemuck::cast_slice(&opaque_vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            self.opaque_model.vertices_buffer = Some(vertex_buffer);
        }

        if translucent_vertices.len() != 0 {
            let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
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
            let indices_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: Some("Chunk Opaque Mesh Data Indices Buffer"),
                contents: &bytemuck::cast_slice(&opaque_indices),
                usage: wgpu::BufferUsages::INDEX,
            });
            self.opaque_model.indices_buffer = Some(indices_buffer);
        }

        if self.translucent_model.indices_buffer_len != 0 {
            let indices_buffer = device.create_buffer_init(&BufferInitDescriptor {
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

        let model_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Chunk translation matrix buffer"),
            contents: &bytemuck::cast_slice(&[model.clone()]),
            usage: wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        let model_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
