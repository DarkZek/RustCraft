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
            object.velocity.y -= 0.5;
        }

        // Stop when going slow enough to save computation
        if object.velocity.norm() < 0.1 {
            object.velocity = Vector3::zeros();
        }

        let mut proposed_delta = object.velocity * time.delta_seconds();
        let proposed_aabb: Aabb = object.collider.offset(object.position + proposed_delta);

        // Stop from falling through world
        let collisions = proposed_aabb.get_voxel_collision_colliders(&chunks, &block_states);

        proposed_aabb.draw(
            &mut draw_lines,
            0.0,
            if collisions.is_empty() {
                Color::GREEN
            } else {
                Color::ORANGE
            },
        );

        for block in collisions {
            block.draw(&mut draw_lines, 0.0, Color::RED);

            println!("{:?}", proposed_delta);
            let mut new_proposed_delta =
                proposed_aabb.try_move(Vector3::new(proposed_delta.x, 0.0, 0.0), &block);
            new_proposed_delta +=
                proposed_aabb.try_move(Vector3::new(0.0, proposed_delta.y, 0.0), &block);
            new_proposed_delta +=
                proposed_aabb.try_move(Vector3::new(0.0, 0.0, proposed_delta.z), &block);

            proposed_delta = new_proposed_delta;

            println!("{:?}", proposed_delta);

            // Remove downward velocity
            //object.velocity.y = 0.0;
        }

        // Proposed delta is small so hit a wall, remove velocity
        if f32::abs(proposed_delta.x) < 0.08 {
            object.velocity.x = 0.0;
        }

        if f32::abs(proposed_delta.y) < 0.08 {
            object.velocity.y = 0.0;
        }

        if f32::abs(proposed_delta.z) < 0.08 {
            object.velocity.z = 0.0;
        }

        object.position += proposed_delta;
    }
}
