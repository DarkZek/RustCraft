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
        object.velocity *= 0.86;

        if object.gravity {
            object.velocity.y -= 15.0 * time.delta_seconds();
        }

        // Stop when going slow enough to save computation
        if object.velocity.norm() < 0.05 {
            object.velocity = Vector3::zeros();
        }

        let mut proposed_delta = object.velocity * time.delta_seconds();
        let proposed_aabb: Aabb = object.collider.offset(object.position + proposed_delta);
        let current_aabb: Aabb = object.collider.offset(object.position);

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
        current_aabb.draw(&mut draw_lines, 0.0, Color::rgba(0.5, 0.5, 0.5, 0.6));

        for block in collisions {
            block.draw(&mut draw_lines, 0.0, Color::RED);

            println!("{:?}", proposed_delta);
            let mut new_proposed_delta = Vector3::zeros();

            if proposed_delta.x != 0.0 {
                new_proposed_delta +=
                    current_aabb.try_move(Vector3::new(proposed_delta.x, 0.0, 0.0), &block);
                println!("N1 {:?}", new_proposed_delta);
            }
            if proposed_delta.y != 0.0 {
                new_proposed_delta +=
                    current_aabb.try_move(Vector3::new(0.0, proposed_delta.y, 0.0), &block);
                println!("N2 {:?}", new_proposed_delta);
            }
            if proposed_delta.z != 0.0 {
                new_proposed_delta +=
                    current_aabb.try_move(Vector3::new(0.0, 0.0, proposed_delta.z), &block);
                println!("N3 {:?}", new_proposed_delta);
            }

            proposed_delta = new_proposed_delta;

            println!("O {:?}", proposed_delta);
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

        object.position += proposed_delta;
    }
}
