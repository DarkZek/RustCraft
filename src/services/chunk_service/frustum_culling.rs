use cgmath::{Vector3, Quaternion, InnerSpace, Euler, Rad, Rotation3};
use crate::render::camera::Camera;
use crate::services::chunk_service::chunk::Chunk;
use crate::services::settings_service::CHUNK_SIZE;
use std::collections::HashMap;
use std::f32::consts::PI;

pub fn calculate_frustum_culling(cam: &Camera, viewable_chunks: &Vec<Vector3<i32>>, chunks: &HashMap<Vector3<i32>, Chunk>) -> Vec<Vector3<i32>> {

    // (Normal, d)
    let mut faces: [(Vector3<f32>, f32); 3] = [
        (Vector3 {
            x: 1.0,
            y: 0.0,
            z: -1.0
        }, 8.0),
        (Vector3 {
            x: 1.0,
            y: 0.0,
            z: 1.0
        }, 8.0),
        (Vector3 {
            x: 1.0,
            y: -1.0,
            z: 0.0
        }, 8.0)
    ];

    //let view: Quaternion<f32> = Quaternion::from(Euler::new(((cam.yaw - PI / 2.0).cos() * cam.pitch.cos()), 0.0, -(cam.yaw - PI / 2.0).sin() * -cam.pitch.cos()).into());

    //cam.pitch.sin() as f32,

    // for (normal, distance) in faces.iter_mut() {
    //     *normal = &view * &normal.clone();
    // }

    let mut loaded_chunks = Vec::new();

    for pos in viewable_chunks {

        let chunk = chunks.get(pos).unwrap();

        let relative_pos = (chunk.position * CHUNK_SIZE as i32);
        let mut relative_pos = Vector3 {
            x: relative_pos.x as f32,
            y: relative_pos.y as f32,
            z: relative_pos.z as f32
        };
        /*
        let mut relative_pos = Vector3 {
            x: relative_pos.x as f32 - cam.eye.x,
            y: relative_pos.y as f32 - cam.eye.y,
            z: relative_pos.z as f32 - cam.eye.z
        };
         */

        if is_visible(relative_pos, 20.0, &faces) {
            loaded_chunks.push(pos.clone());
        }
    }

    loaded_chunks
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
    (v1.x * v2.x) +
    (v1.y * v2.y) +
    (v1.z * v2.z)
}