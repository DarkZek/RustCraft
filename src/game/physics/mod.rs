use crate::game::physics::collider::BoxCollider;
use crate::helpers::{chunk_by_loc_from_read, Clamp};
use crate::services::chunk_service::chunk::{ChunkData, RawChunkData};
use crate::services::settings_service::CHUNK_SIZE;
use nalgebra::Vector3;
use specs::{Component, ReadStorage, System, VecStorage, Write, WriteStorage};
use std::time::{Instant, SystemTime};

pub mod collider;
pub mod interpolator;
pub mod system;

pub struct Physics {
    pub slipperiness: f32,
    pub gravity: f32,
    pub drag: f32,
    pub updates_per_second: u32,
    pub updates: u32,
    pub last_second: SystemTime,
}

impl Physics {
    pub fn new() -> Physics {
        Physics {
            slipperiness: 0.06,
            gravity: 0.08,
            drag: 0.02,
            updates_per_second: 0,
            updates: 0,
            last_second: SystemTime::now(),
        }
    }
}

impl Default for Physics {
    fn default() -> Self {
        Physics::new()
    }
}

pub struct PhysicsProcessingSystem;

impl<'a> System<'a> for PhysicsProcessingSystem {
    type SystemData = (
        WriteStorage<'a, PhysicsObject>,
        ReadStorage<'a, ChunkData>,
        Write<'a, Physics>,
    );

    fn run(&mut self, (mut physics_objects, chunks, mut physics): Self::SystemData) {
        use crate::helpers::TryParJoin;

        #[cfg(not(target_arch = "wasm32"))]
        use specs::prelude::ParallelIterator;

        // Update rate
        if physics.last_second.elapsed().unwrap().as_millis() > 1000 {
            physics.updates_per_second = physics.updates;
            physics.updates = 0;
            physics.last_second = SystemTime::now();
        } else {
            physics.updates += 1;
        }

        (&mut physics_objects).try_par_join().for_each(|entity| {
            // Check collisions
            let slipperiness = 0.6;

            entity.velocity.x *= physics.slipperiness;
            entity.velocity.z *= physics.slipperiness;

            // Add gravity
            entity.velocity.y -= physics.gravity;

            // Air Drag
            entity.velocity.y *= (1.0 - physics.drag);

            let movement = move_entity_xyz(
                &entity.collider,
                &chunks,
                &mut entity.velocity,
                entity.position,
            );

            entity.touching_ground = movement.y == 0.0;

            if entity.touching_ground {
                if entity.velocity.y < 0.0 {
                    entity.velocity.y = 0.0;
                }
            } else {
                // Terminal velocity
                if entity.velocity.y < -3.92 {
                    entity.velocity.y = -3.92;
                }
            }

            entity.old_position = entity.position;
            entity.new_position = entity.position + movement;
        });
    }
}

#[derive(Debug)]
pub struct PhysicsObject {
    pub velocity: Vector3<f32>,
    pub position: Vector3<f32>,
    pub old_position: Vector3<f32>,
    pub new_position: Vector3<f32>,
    pub collider: BoxCollider,
    pub touching_ground: bool,
}

impl Component for PhysicsObject {
    type Storage = VecStorage<Self>;
}

impl PhysicsObject {
    pub fn new() -> PhysicsObject {
        PhysicsObject {
            velocity: Vector3::new(0.0, 0.0, 0.0),
            position: Vector3::new(0.0, 70.0, 0.0),
            old_position: Vector3::new(0.0, 70.0, 0.0),
            new_position: Vector3::new(0.0, 70.0, 0.0),
            collider: BoxCollider::blank(),
            touching_ground: false,
        }
    }
}

//TODO: Address physics issues when moving between chunks

// Calculates movement based on velocity and colliders
pub fn move_entity_xyz(
    collider: &BoxCollider,
    chunks: &ReadStorage<ChunkData>,
    velocity: &mut Vector3<f32>,
    absolute_position: Vector3<f32>,
) -> Vector3<f32> {
    let chunk_pos = absolute_pos_to_chunk(absolute_position);

    let (y_change, y_collided) = if let Some(chunk) = chunk_by_loc_from_read(&chunks, chunk_pos) {
        move_entity_dir(collider, &chunk.world, Vector3::new(0.0, velocity.y, 0.0))
    } else {
        (velocity.y, false)
    };
    if y_collided {
        velocity.y = 0.0;
    }

    let z_change = if let Some(data) = chunk_by_loc_from_read(chunks, chunk_pos) {
        let (z_change, z_collided) =
            move_entity_dir(collider, &data.world, Vector3::new(0.0, 0.0, velocity.z));
        if z_collided {
            velocity.z = 0.0;
        }
        z_change
    } else {
        velocity.z
    };

    let x_change = if let Some(data) = chunk_by_loc_from_read(chunks, chunk_pos) {
        let (x_change, x_collided) =
            move_entity_dir(collider, &data.world, Vector3::new(velocity.x, 0.0, 0.0));
        if x_collided {
            velocity.x = 0.0;
        }
        x_change
    } else {
        velocity.x
    };

    Vector3::new(x_change, y_change, z_change)
}

/// Move an entity on a single axis, if any points that arent colliding collide after the move, cancel the move
fn move_entity_dir(
    collider: &BoxCollider,
    chunk: &RawChunkData,
    movement: Vector3<f32>,
) -> (f32, bool) {
    let start_collisions = count_collisions(collider, chunk);

    let new_collider = BoxCollider {
        p1: collider.p1 + movement,
        p2: collider.p2 + movement,
        center: collider.center + movement,
    };

    let end_collisions = count_collisions(&new_collider, chunk);

    // Can make full move
    if start_collisions >= end_collisions {
        // Only one of these will have a value so just print them all
        (movement.x + movement.y + movement.z, false)
    } else {
        (0.0, true)
    }
}

/// Counts how many points (out of 8) are colliding
#[cfg_attr(rustfmt, rustfmt_skip)]
fn count_collisions(collider: &BoxCollider, chunk: &RawChunkData) -> i32 {
    let mut collisions = 0;
    if is_colliding(Vector3::new(collider.p1.x, collider.p1.y, collider.p1.z), chunk) { collisions += 1; }
    if is_colliding(Vector3::new(collider.p2.x, collider.p1.y, collider.p1.z), chunk) { collisions += 1; }
    if is_colliding(Vector3::new(collider.p1.x, collider.p1.y, collider.p2.z), chunk) { collisions += 1; }
    if is_colliding(Vector3::new(collider.p2.x, collider.p1.y, collider.p2.z), chunk) { collisions += 1; }
    if is_colliding(Vector3::new(collider.p2.x, collider.p2.y, collider.p2.z), chunk) { collisions += 1; }
    if is_colliding(Vector3::new(collider.p1.x, collider.p2.y, collider.p2.z), chunk) { collisions += 1; }
    if is_colliding(Vector3::new(collider.p2.x, collider.p2.y, collider.p1.z), chunk) { collisions += 1; }
    if is_colliding(Vector3::new(collider.p1.x, collider.p2.y, collider.p1.z), chunk) { collisions += 1; }
    collisions
}

fn is_colliding(point: Vector3<f32>, chunk: &RawChunkData) -> bool {
    let block = Vector3::new(
        point.x.floor().clamp_val(0.0, 15.0) as i32,
        point.y.floor().clamp_val(0.0, 15.0) as i32,
        point.z.floor().clamp_val(0.0, 15.0) as i32,
    );
    chunk[block.x as usize][block.y as usize][block.z as usize] != 0
}

fn absolute_pos_to_chunk(pos: Vector3<f32>) -> Vector3<i32> {
    Vector3::new(
        (pos.x / CHUNK_SIZE as f32).floor() as i32,
        (pos.y / CHUNK_SIZE as f32).floor() as i32,
        (pos.z / CHUNK_SIZE as f32).floor() as i32,
    )
}
