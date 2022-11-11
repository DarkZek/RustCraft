use crate::services::physics::simulate::physics_tick;
use crate::services::physics::sync::physics_sync;
use bevy::ecs::component::Component;
use bevy::prelude::{App, Plugin};
use nalgebra::Vector3;

pub mod raycasts;
mod simulate;
mod sync;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(physics_tick).add_system(physics_sync);
    }
}

#[derive(Component)]
pub struct PhysicsObject {
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
}

impl PhysicsObject {
    pub fn new(position: Vector3<f32>) -> PhysicsObject {
        PhysicsObject {
            position,
            velocity: Vector3::zeros(),
        }
    }
}
