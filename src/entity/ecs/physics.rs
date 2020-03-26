use specs::{System, WriteStorage};
use crate::entity::ecs::{Position, Velocity};
use specs::Join;

pub struct PhysicsSystem;

const DRAG: f32 = 0.4;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (WriteStorage<'a, Velocity>,
                       WriteStorage<'a, Position>);

    fn run(&mut self, (mut vel, mut pos): Self::SystemData) {
        let delta_time = 0.2;
        for (vel, pos) in (&mut vel, &mut pos).join() {
            pos.x += vel.x * delta_time;
            pos.y += vel.y * delta_time;
            pos.z += vel.z * delta_time;

            vel.x *= 1.0 - DRAG;
            vel.y *= 1.0 - DRAG;
            vel.z *= 1.0 - DRAG;
        }
    }
}