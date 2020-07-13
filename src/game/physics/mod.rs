use crate::game::physics::collider::{BoxCollider, CollisionSide};
use crate::services::chunk_service::ChunkService;
use crate::services::settings_service::CHUNK_SIZE;
use nalgebra::Vector3;
use specs::{Component, Read, System, VecStorage, WriteStorage, ParJoin};
use specs::prelude::ParallelIterator;
use crate::services::chunk_service::chunk::Chunk;

pub mod collider;
pub mod interpolator;

pub struct PhysicsProcessingSystem;

impl<'a> System<'a> for PhysicsProcessingSystem {
    type SystemData = (WriteStorage<'a, PhysicsObject>, Read<'a, ChunkService>);

    fn run(&mut self, (mut physics_objects, chunk_service): Self::SystemData) {
        use specs::Join;

        (&mut physics_objects).par_join()
            .for_each(|entity| {

                // Check collisions
                let chunk_pos = Vector3::new(
                    (entity.position.x / CHUNK_SIZE as f32).floor() as i32,
                    (entity.position.y / CHUNK_SIZE as f32).floor() as i32,
                    (entity.position.z / CHUNK_SIZE as f32).floor() as i32,
                );

                let chunk = match chunk_service.chunks.get(&chunk_pos) {
                    Some(val) => val,
                    // If its in an unloaded it should just float
                    none => return,
                };

                //TODO: Greedy mesh the frick out of the colliders

                let mut collision_target = CollisionSide::zero();

                if let Chunk::Tangible(chunk) = chunk {
                    for collider in &chunk.collision_map {
                        let collision = collider.check_collision(&entity.collider);
                        if collision.is_some() {
                            collision_target.combine(&collision.unwrap());
                            break;
                        }
                    }
                }

                let slipperiness = 0.6;

                entity.velocity.x *= slipperiness;
                entity.velocity.z *= slipperiness;

                // Add gravity
                entity.velocity.y -= 0.08;

                // Air Drag
                entity.velocity.y *= 0.98;

                if !collision_target.bottom {
                    entity.touching_ground = false;

                    // Terminal velocity
                    if entity.velocity.y < -3.92 {
                        entity.velocity.y = -3.92;
                    }
                } else {
                    entity.touching_ground = true;

                    if entity.velocity.y < 0.0 {
                        entity.velocity.y = 0.0;
                    }
                }

                // if collision_target.front && entity.velocity.x > 0.0 { entity.velocity.x = 0.0;}
                // if collision_target.back && entity.velocity.x < 0.0 { entity.velocity.x = 0.0;}
                // if collision_target.left && entity.velocity.z > 0.0 { entity.velocity.z = 0.0;}
                // if collision_target.right && entity.velocity.z < 0.0 { entity.velocity.z = 0.0;}

                entity.old_position = entity.position;
                entity.new_position = entity.position + entity.velocity;
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
            position: Vector3::new(0.0, 90.0, 0.0),
            old_position: Vector3::new(0.0, 90.0, 0.0),
            new_position: Vector3::new(0.0, 90.0, 0.0),
            collider: BoxCollider::blank(),
            touching_ground: false,
        }
    }
}
