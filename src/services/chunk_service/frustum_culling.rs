use crate::render::camera::Camera;
use crate::render::chunks_render_bundle::ChunksRenderBundle;
use crate::render::RenderState;
use crate::services::asset_service::AssetService;
use crate::services::chunk_service::chunk::{ChunkData, Chunks};
use crate::services::chunk_service::ChunkService;
use crate::services::settings_service::CHUNK_SIZE;
use futures::StreamExt;
use nalgebra::{ArrayStorage, Matrix, Matrix4, Vector3, U1, U4};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use specs::{Join, ParJoin};
use specs::{Read, ReadStorage, System, Write};

pub struct FrustumCullingSystem;

impl<'a> System<'a> for FrustumCullingSystem {
    type SystemData = (
        Write<'a, ChunkService>,
        Read<'a, Camera>,
        ReadStorage<'a, ChunkData>,
        Write<'a, ChunksRenderBundle>,
        Read<'a, RenderState>,
        Read<'a, AssetService>,
    );

    fn run(
        &mut self,
        (mut chunk_service, camera, chunks, mut chunks_render_bundle, render_state, asset_service): Self::SystemData,
    ) {
        if chunk_service.update_frustum_culling(&camera, &chunks) {
            // Redo the render bundle
            println!("Re-doing bundle");
            chunks_render_bundle.create_render_bundle(
                &render_state,
                &asset_service,
                &chunk_service,
                &chunks,
            );
        }
    }
}

pub fn calculate_frustum_culling(
    cam: &Camera,
    chunks: &ReadStorage<ChunkData>,
) -> Vec<Vector3<i32>> {
    let opengl_to_wgpu_matrix: Matrix4<f32> = Matrix4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
    );
    //let chunks = Chunks::new(chunks.join().collect::<Vec<&ChunkData>>());
    let frustum = Frustum::from_matrix(opengl_to_wgpu_matrix * cam.proj * cam.view);

    if let None = frustum {
        return Vec::new();
    }

    let frustum = frustum.unwrap();

    let chunks = chunks
        .par_join()
        .map(|chunk| {
            let relative_pos = chunk.position * CHUNK_SIZE as i32;
            let relative_pos = Vector3::new(
                relative_pos.x as f32 - cam.eye.x,
                relative_pos.y as f32 - cam.eye.y,
                relative_pos.z as f32 - cam.eye.z,
            );

            if frustum.is_visible(relative_pos, 25.0) {
                if chunk.opaque_model.vertices_buffer.is_some()
                    || chunk.translucent_model.vertices_buffer.is_some()
                {
                    return Some(chunk.position.clone());
                }
            }
            None
        })
        .collect::<Vec<Option<Vector3<i32>>>>();

    let mut visible_chunks = Vec::new();

    for chunk in chunks {
        if let Some(val) = chunk {
            visible_chunks.push(val);
        }
    }

    visible_chunks
}

fn dot(v1: Vector3<f32>, v2: Vector3<f32>) -> f32 {
    (v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z)
}

// Inspiration taken from https://docs.rs/collision/0.20.1/src/collision/frustum.rs.html

#[derive(Clone, Copy)]
pub struct Plane {
    pub n: Vector3<f32>,
    pub d: f32,
}

impl Plane {
    pub fn from_vector4(v: Matrix<f32, U1, U4, ArrayStorage<f32, U1, U4>>) -> Plane {
        Plane {
            n: Vector3::new(v.x, v.y, v.z),
            d: -v.w,
        }
    }

    pub fn normalize(&self) -> Option<Plane> {
        if self.n == Vector3::zeros() {
            None
        } else {
            let denom = 1.0 / self.n.magnitude();
            Some(Plane {
                n: self.n * denom,
                d: self.d * denom,
            })
        }
    }
}

#[derive(Copy, Clone)]
struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    pub fn from_matrix(matrix: Matrix4<f32>) -> Option<Frustum> {
        let data = [
            matrix.row(3) + matrix.row(0),
            matrix.row(3) - matrix.row(0),
            matrix.row(3) + matrix.row(1),
            matrix.row(3) - matrix.row(1),
            matrix.row(3) + matrix.row(2),
            matrix.row(3) - matrix.row(2),
        ];

        let mut planes = [Plane {
            n: Vector3::zeros(),
            d: 0.0,
        }; 6];

        for i in 0..6 {
            planes[i] = match Plane::from_vector4(data[i]).normalize() {
                Some(p) => p,
                None => return None,
            };
        }

        Some(Frustum { planes })
    }

    pub fn is_visible(&self, center: Vector3<f32>, radius: f32) -> bool {
        for i in 0..self.planes.len() {
            if dot(center, self.planes[i].n) as f32 + radius <= 0.0 {
                return false;
            }
        }

        return true;
    }
}
