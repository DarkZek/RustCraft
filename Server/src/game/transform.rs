use bevy_ecs::prelude::*;
use nalgebra::{Quaternion, Vector3};
use std::path::Component;

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
