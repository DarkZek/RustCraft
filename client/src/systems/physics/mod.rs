use crate::state::AppState;
use crate::systems::chunk::ChunkSystem;
use crate::systems::physics::simulate::physics_tick;
use crate::systems::physics::sync::physics_sync;
use bevy::ecs::component::Component;
use bevy::prelude::{in_state, FixedUpdate, IntoSystemConfigs};
use bevy::prelude::{App, Plugin};
use nalgebra::Vector3;
use rc_shared::aabb::Aabb;
use rc_shared::block::BlockStates;

pub mod raycasts;
mod simulate;
mod sync;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (physics_tick, physics_sync)
                .chain()
                .run_if(in_state(AppState::InGame)),
        );
    }
}

/// Stores physics related properties of an object in the world
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
    /// Creates a new physics object
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

    /// Translates a physics object by a delta, with delta position collision detection
    pub fn translate_with_collision_detection(
        &mut self,
        delta: Vector3<f32>,
        chunks: &ChunkSystem,
        block_states: &BlockStates,
    ) {
        if delta == Vector3::zeros() {
            return;
        }

        let mut current_aabb = self.collider.offset(self.position);
        let potential_collisions =
            current_aabb.get_surrounding_voxel_collision_colliders(chunks, &block_states);

        if delta.x != 0.0 {
            self.position +=
                current_aabb.try_translate(Vector3::new(delta.x, 0.0, 0.0), &potential_collisions);
            current_aabb = self.collider.offset(self.position);
        }
        if delta.y != 0.0 {
            self.position +=
                current_aabb.try_translate(Vector3::new(0.0, delta.y, 0.0), &potential_collisions);
            current_aabb = self.collider.offset(self.position);
        }
        if delta.z != 0.0 {
            self.position +=
                current_aabb.try_translate(Vector3::new(0.0, 0.0, delta.z), &potential_collisions);
        }
    }
}
