use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Aabb {
    top_left: Vector3<f32>,
    size: Vector3<f32>,
}

impl Aabb {
    pub fn new(top_left: Vector3<f32>, size: Vector3<f32>) -> Aabb {
        Aabb { top_left, size }
    }
}
