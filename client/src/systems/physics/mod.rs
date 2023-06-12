use crate::systems::physics::aabb::Aabb;
use crate::systems::physics::simulate::physics_tick;
use crate::systems::physics::sync::physics_sync;
use bevy::ecs::component::Component;
use bevy::prelude::{App, Plugin};
use nalgebra::Vector3;

pub mod aabb;
pub mod raycasts;
mod simulate;
mod sync;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        //app.add_system(physics_tick).add_system(physics_sync);
    }
}

#[derive(Component)]
pub struct PhysicsObject {
    pub position: Vector3<f32>,
    pub previous_position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub collider: Aabb,
    pub gravity: bool,
    pub touching_ground: bool,
}

impl PhysicsObject {
    pub fn new(position: Vector3<f32>, collider: Aabb) -> PhysicsObject {
        PhysicsObject {
            position,
            previous_position: position,
            velocity: Vector3::zeros(),
            collider,
            gravity: false,
            touching_ground: false,
        }
    }
}
