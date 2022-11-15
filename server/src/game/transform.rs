use bevy::ecs::prelude::*;
use nalgebra::{Quaternion, Vector3};


#[derive(Component)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: Vector3::zeros(),
            rotation: Quaternion::default(),
        }
    }
}
