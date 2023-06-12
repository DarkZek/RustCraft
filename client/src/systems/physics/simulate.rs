use crate::game::blocks::states::BlockStates;
use crate::helpers::global_to_local_position;
use crate::systems::chunk::data::ChunkData;
use crate::systems::chunk::ChunkSystem;
use crate::systems::physics::aabb::Aabb;
use crate::systems::physics::PhysicsObject;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use nalgebra::{clamp, Vector3};

pub fn physics_tick(
    mut query: Query<&mut PhysicsObject>,
    chunks: Res<ChunkSystem>,
    block_states: Res<BlockStates>,
    time: Res<Time>,
    mut draw_lines: ResMut<DebugLines>,
) {
    for mut object in query.iter_mut() {
        object.previous_position = object.position;
        object.velocity *= 0.92;

        if object.gravity {
            object.velocity.y -= 50.0 * time.delta_seconds();
        }

        let mut proposed_delta = object.velocity * time.delta_seconds();
        let mut current_aabb: Aabb = object.collider.offset(object.position);
        current_aabb.draw(&mut draw_lines, 0.0, Color::GREEN);

        // Stop from falling through world
        let potential_collisions =
            current_aabb.get_surrounding_voxel_collision_colliders(&chunks, &block_states);

        if proposed_delta.x != 0.0 {
            object.position += translate_entity(
                current_aabb,
                Vector3::new(proposed_delta.x, 0.0, 0.0),
                &potential_collisions,
                &mut draw_lines,
            );
            current_aabb = object.collider.offset(object.position);
        }
        if proposed_delta.y != 0.0 {
            object.position += translate_entity(
                current_aabb,
                Vector3::new(0.0, proposed_delta.y, 0.0),
                &potential_collisions,
                &mut draw_lines,
            );
            current_aabb = object.collider.offset(object.position);
        }
        if proposed_delta.z != 0.0 {
            object.position += translate_entity(
                current_aabb,
                Vector3::new(0.0, 0.0, proposed_delta.z),
                &potential_collisions,
                &mut draw_lines,
            );
        }

        // Proposed delta is small so hit a wall, remove velocity
        if f32::abs(proposed_delta.x) < 0.001 {
            object.velocity.x = 0.0;
        }

        if f32::abs(proposed_delta.y) < 0.001 {
            object.velocity.y = 0.0;
        }

        if f32::abs(proposed_delta.z) < 0.001 {
            object.velocity.z = 0.0;
        }

        if translate_entity(
            current_aabb,
            Vector3::new(0.0, 0.05, 0.0),
            &potential_collisions,
            &mut draw_lines,
        )
        .y == 0.05
        {
            // Within 0.05 blocks of the ground
            object.touching_ground = true;
        }
    }
}

/// One axis at a time
fn translate_entity(
    current_aabb: Aabb,
    mut proposed_delta: Vector3<f32>,
    colliders: &Vec<Aabb>,
    draw_lines: &mut DebugLines,
) -> Vector3<f32> {
    let mut proposed_aabb = current_aabb.offset(proposed_delta);

    for block in colliders {
        if !block.aabb_collides(&proposed_aabb) {
            continue;
        }
        block.draw(draw_lines, 0.0, Color::RED);
        // Previous delta change could have made the move redundent
        if block.aabb_collides(&proposed_aabb) {
            proposed_delta = current_aabb.try_move(proposed_delta, &block);
        }
        proposed_aabb = current_aabb.offset(proposed_delta);
    }

    proposed_delta
}
