use specs::{System, WriteStorage, Read, ParJoin};
use crate::game::physics::PhysicsObject;
use nalgebra::{Vector3};
use crate::helpers::lerp;
use specs::prelude::ParallelIterator;

pub struct PhysicsInterpolationFactor(pub f32);

impl Default for PhysicsInterpolationFactor {
    fn default() -> Self {
        PhysicsInterpolationFactor(0.0)
    }
}

pub struct PhysicsInterpolationSystem;

impl<'a> System<'a> for PhysicsInterpolationSystem {
    type SystemData = (WriteStorage<'a, PhysicsObject>, Read<'a, PhysicsInterpolationFactor>);

    fn run(&mut self, (mut physics_objects, interpolation_factor): Self::SystemData) {
        use specs::Join;

        (&mut physics_objects).par_join()
            .for_each(|entity| {
                entity.position = lerp_vec3(&entity.old_position, &entity.new_position, interpolation_factor.0);
            });
    }
}

fn lerp_vec3(pos1: &Vector3<f32>, pos2: &Vector3<f32>, amount: f32) -> Vector3<f32> {
    Vector3::new(
        lerp(pos1.x, pos2.x, amount),
        lerp(pos1.y, pos2.y, amount),
        lerp(pos1.z, pos2.z, amount),
    )
}