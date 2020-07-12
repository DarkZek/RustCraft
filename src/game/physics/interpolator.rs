use specs::{System, WriteStorage, Read, ParJoin};
use crate::game::physics::PhysicsObject;
use nalgebra::{Vector3};
use specs::prelude::ParallelIterator;
use crate::helpers::Lerp;

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
        (&mut physics_objects).par_join()
            .for_each(|entity| {
                entity.position = lerp_vec3(&entity.old_position, &entity.new_position, interpolation_factor.0);
            });
    }
}

fn lerp_vec3(pos1: &Vector3<f32>, pos2: &Vector3<f32>, amount: f32) -> Vector3<f32> {
    Vector3::new(
        pos1.x.lerp(pos2.x, amount),
        pos1.y.lerp(pos2.y, amount),
        pos1.z.lerp(pos2.z, amount),
    )
}