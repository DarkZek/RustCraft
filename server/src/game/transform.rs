use bevy::ecs::prelude::*;
use nalgebra::{Quaternion, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
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

impl Transform {
    pub fn from_translation(position: Vector3<f32>) -> Transform {
        Transform {
            position,
            rotation: Default::default(),
        }
    }
}
