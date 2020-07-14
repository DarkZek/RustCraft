use crate::render::camera::Camera;
use crate::services::chunk_service::chunk::Chunk;
use crate::services::chunk_service::ChunkService;
use crate::services::settings_service::CHUNK_SIZE;
use nalgebra::{ArrayStorage, Matrix, Vector3, U1, U3};
use specs::{Read, System, Write};
use std::collections::HashMap;
use std::f32::consts::PI;

pub struct FrustumCullingSystem;

impl<'a> System<'a> for FrustumCullingSystem {
    type SystemData = (Write<'a, ChunkService>, Read<'a, Camera>);

    fn run(&mut self, (mut chunk_service, camera): Self::SystemData) {
        chunk_service.update_frustum_culling(&camera);
    }
}

pub fn calculate_frustum_culling(
    _cam: &Camera,
    viewable_chunks: &Vec<Vector3<i32>>,
    chunks: &HashMap<Vector3<i32>, Chunk>,
) -> Vec<Vector3<i32>> {
    let rot_y = -_cam.yaw + (PI / 2.0);
    let rot_z = _cam.pitch;

    let left_pane = rotate_y_axis(Vector3::new(1.0, 0.0, -1.0), rot_y);
    let right_pane = rotate_y_axis(Vector3::new(1.0, 0.0, 1.0), rot_y);
    let bottom_pane = rotate_y_axis(Vector3::new(1.0, -1.0, 0.0), rot_y);

    // (Normal, d)
    let faces: [(Vector3<f32>, f32); 3] = [
        (left_pane, 0.0),
        (right_pane, 0.0),
        (bottom_pane, 0.0)
    ];

    let mut visible_chunks = Vec::new();

    for pos in viewable_chunks {
        let chunk = chunks.get(pos).unwrap();

        if let Chunk::Tangible(chunk) = chunk {
            let relative_pos = chunk.position * CHUNK_SIZE as i32;
            let relative_pos = Vector3::new(
                relative_pos.x as f32 - _cam.eye.x,
                relative_pos.y as f32 - _cam.eye.y,
                relative_pos.z as f32 - _cam.eye.z,
            );

            if chunk.vertices_buffer.is_some() && chunk.indices_buffer.is_some() {
                if is_visible(relative_pos, 23.0, &faces) {
                    visible_chunks.push(pos.clone());
                }
            }
        }
    }

    visible_chunks
}

fn rotate_y_axis(pos: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>, rotation: f32) -> Vector3<f32> {
    Vector3::new(
        (pos.x * rotation.cos()) + (pos.z * rotation.sin()),
        pos.y,
        (-pos.x * rotation.sin()) + (pos.z * rotation.cos()),
    )
}

fn rotate_z_axis(pos: Vector3<f32>, rotation: f32) -> Vector3<f32> {
    Vector3::new(
        (pos.x * rotation.cos()) - (pos.y * rotation.sin()),
        (pos.x * rotation.sin()) + (pos.y * rotation.cos()),
        pos.z,
    )
}

fn rotate_x_axis(pos: Vector3<f32>, rotation: f32) -> Vector3<f32> {
    Vector3::new(
        pos.x,
        (pos.y * rotation.cos()) - (pos.z * rotation.sin()),
        (pos.y * rotation.sin()) + (pos.z * rotation.cos()),
    )
}

pub fn is_visible(center: Vector3<f32>, radius: f32, faces: &[(Vector3<f32>, f32); 3]) -> bool {
    for i in 0..faces.len() {
        if dot(center, faces[i].0) as f32 + faces[i].1 + radius <= 0.0 {
            return false;
        }
    }

    return true;
}

fn dot(v1: Vector3<f32>, v2: Vector3<f32>) -> f32 {
    (v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z)
}
