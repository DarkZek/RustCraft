use crate::render::camera::Camera;
use crate::services::chunk_service::chunk::{Chunk, Chunks};
use crate::services::chunk_service::ChunkService;
use crate::services::settings_service::CHUNK_SIZE;
use nalgebra::{ArrayStorage, Matrix, Matrix4, Vector3, U1, U4};
use specs::{Read, System, Write};
use std::collections::HashMap;

pub struct FrustumCullingSystem;

impl<'a> System<'a> for FrustumCullingSystem {
    type SystemData = (Write<'a, ChunkService>, Read<'a, Camera>, Read<'a, Chunks>);

    fn run(&mut self, (mut chunk_service, camera, chunks): Self::SystemData) {
        chunk_service.update_frustum_culling(&camera, &chunks);
    }
}

pub fn calculate_frustum_culling(
    cam: &Camera,
    viewable_chunks: &Vec<Vector3<i32>>,
    chunks: &HashMap<Vector3<i32>, Chunk>,
) -> Vec<Vector3<i32>> {
    let frustum = Frustum::from_matrix(cam.projection_matrix);

    if let None = frustum {
        return Vec::new();
    }

    let frustum = frustum.unwrap();

    let mut visible_chunks = Vec::new();

    for pos in viewable_chunks {
        let chunk = chunks.get(pos).unwrap();

        if let Chunk::Tangible(chunk) = chunk {
            let relative_pos = chunk.position * CHUNK_SIZE as i32;
            let relative_pos = Vector3::new(
                relative_pos.x as f32 - cam.eye.x,
                relative_pos.y as f32 - cam.eye.y,
                relative_pos.z as f32 - cam.eye.z,
            );

            if chunk.opaque_model.vertices_buffer.is_some()
                || chunk.translucent_model.vertices_buffer.is_some()
            {
                if frustum.is_visible(relative_pos, 22.0) {
                    visible_chunks.push(pos.clone());
                }
            }
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
