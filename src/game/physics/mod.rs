use crate::game::physics::collider::BoxCollider;
use crate::services::chunk_service::chunk::{ChunkData, ChunkEntityLookup};
use crate::world::WorldChunks;
use nalgebra::Vector3;
use specs::{Component, Read, ReadStorage, System, VecStorage, Write, WriteStorage};
use std::time::SystemTime;

pub mod collider;
pub mod interpolator;
pub mod player;

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
            gravity: 0.15,
            // gravity: 0.0,
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
        Read<'a, ChunkEntityLookup>,
    );

    fn run(
        &mut self,
        (mut physics_objects, chunks, mut physics, chunk_entity_lookup): Self::SystemData,
    ) {
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
            let _slipperiness = 0.6;

            entity.velocity.x *= physics.slipperiness;
            entity.velocity.z *= physics.slipperiness;

            // Add gravity
            entity.velocity.y -= physics.gravity;

            // Air Drag
            entity.velocity.y *= 1.0 - physics.drag;

            entity.old_position = entity.position;
            entity.new_position = entity.position;

            // Apply velocity
            let (movement, _collision) = move_entity_dir(
                &entity.collider,
                &chunks,
                &chunk_entity_lookup,
                Vector3::new(entity.velocity.x, 0.0, 0.0),
                entity.position,
            );

            entity.new_position += movement;

            // Apply velocity
            let (movement, _collision) = move_entity_dir(
                &entity.collider,
                &chunks,
                &chunk_entity_lookup,
                Vector3::new(0.0, 0.0, entity.velocity.z),
                entity.position,
            );

            entity.new_position += movement;

            // Apply velocity
            let (movement, collision) = move_entity_dir(
                &entity.collider,
                &chunks,
                &chunk_entity_lookup,
                Vector3::new(0.0, entity.velocity.y, 0.0),
                entity.position,
            );

            entity.new_position += movement;

            entity.touching_ground = collision;

            if entity.touching_ground {
                if entity.velocity.y < 0.0 {
                    entity.velocity.y = 0.0;
                }
            } else {
                // Terminal velocity
                if entity.velocity.y < -3.0 {
                    entity.velocity.y = -0.3;
                }
            }
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

/// Move an entity on a single axis, if any points that arent colliding collide after the move, cancel the move
fn move_entity_dir(
    collider: &BoxCollider,
    chunks: &ReadStorage<ChunkData>,
    lookup: &Read<ChunkEntityLookup>,
    dir: Vector3<f32>,
    position: Vector3<f32>,
) -> (Vector3<f32>, bool) {
    let mut bounds = collider.shift(dir + position);

    let min_x = (bounds.min.x - 2.0) as i64;
    let min_y = (bounds.min.y - 2.0) as i64;
    let min_z = (bounds.min.z - 2.0) as i64;
    let max_x = (bounds.max.x + 2.0) as i64;
    let max_y = (bounds.max.y + 2.0) as i64;
    let max_z = (bounds.max.z + 2.0) as i64;

    let mut world_chunks = WorldChunks::new(chunks, lookup);

    let mut hit = false;
    for y in min_y..max_y {
        for z in min_z..max_z {
            for x in min_x..max_x {
                if let Some(block) = world_chunks.get_block(Vector3::new(x, y, z)) {
                    if !block.get_collidable() {
                        continue;
                    }
                    for bounding_box in block.get_collision_boxes() {
                        let bounding_box =
                            bounding_box.shift(Vector3::new(x as f32, y as f32, z as f32));

                        if bounding_box.collides(&bounds) {
                            bounds = bounds.move_out_of(bounding_box, dir);
                            hit = true;
                        }
                    }
                }
            }
        }
    }

    let new_position = bounds.shift(-position);

    (collider.delta(new_position), hit)
}
