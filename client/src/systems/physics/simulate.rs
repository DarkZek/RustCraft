use crate::game::blocks::states::BlockStates;

use crate::systems::chunk::ChunkSystem;
use crate::systems::physics::PhysicsObject;
use crate::systems::ui::debugging::DebuggingUIData;
use bevy::prelude::*;
use nalgebra::Vector3;

const MAX_TOUCHING_GROUND_DIST: f32 = 0.05;
const GRAVITY_STRENGTH: f32 = 50.0;

const GROUND_FRICTION: f32 = 8.0;
const AIR_FRICTION: f32 = 0.4;

pub fn physics_tick(
    mut query: Query<&mut PhysicsObject>,
    chunks: Res<ChunkSystem>,
    block_states: Res<BlockStates>,
    time: Res<Time>,
    mut debugging_uidata: ResMut<DebuggingUIData>,
) {
    // Debug how many ticks per second
    debugging_uidata.physics_ticks += 1;

    for mut object in query.iter_mut() {
        object.previous_position = object.position;

        if object.gravity {
            object.velocity.y -= GRAVITY_STRENGTH * time.delta_seconds();
        }

        let proposed_delta = object.velocity * time.delta_seconds();

        object.translate_with_collision_detection(proposed_delta, &chunks, &block_states);

        // Proposed delta is small so hit a wall, remove velocity
        if f32::abs(object.previous_position.x - object.position.x) < 0.001 {
            object.velocity.x = 0.0;
        }

        if f32::abs(object.previous_position.y - object.position.y) < 0.001 {
            object.velocity.y = 0.0;
        }

        if f32::abs(object.previous_position.z - object.position.z) < 0.001 {
            object.velocity.z = 0.0;
        }

        let current_aabb = object.collider.offset(object.position);

        let potential_collisions =
            current_aabb.get_surrounding_voxel_collision_colliders(&chunks, &block_states);

        object.touching_ground = current_aabb
            .try_translate(
                Vector3::new(0.0, -MAX_TOUCHING_GROUND_DIST, 0.0),
                &potential_collisions,
            )
            .y
            > -MAX_TOUCHING_GROUND_DIST;

        if object.touching_ground {
            object.velocity *= 1.0 - (GROUND_FRICTION * time.delta_seconds());
        } else {
            object.velocity *= 1.0 - (AIR_FRICTION * time.delta_seconds());
        }
    }
}
