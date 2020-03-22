use specs::{Builder, Component, ReadStorage, System, VecStorage, World, WorldExt, RunNow, WriteStorage};
use crate::entity::ecs::{Position, Velocity, Rotation, Player};
use specs::Join;
use cgmath::{Quaternion, Vector3};

pub struct MovementSystem {
    pub rotation: Option<Quaternion<f32>>,
    pub position: Option<Vector3<f32>>,
    pub velocity: Option<Vector3<f32>>
}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Velocity>,
                       WriteStorage<'a, Position>,
                       WriteStorage<'a, Rotation>,
                       ReadStorage<'a, Player>);

    fn run(&mut self, (mut vel, mut pos, mut rot, player): Self::SystemData) {
        for (vel, pos, rot, player) in (&mut vel, &mut pos, &mut rot, &player).join() {
            if player.local {
                //This is the player
                if let Some(new_pos) = self.position {
                    pos.x = new_pos.x;
                    pos.y = new_pos.y;
                    pos.z = new_pos.z;
                }

                if let Some(new_vel) = self.velocity {
                    vel.x = new_vel.x;
                    vel.y = new_vel.y;
                    vel.z = new_vel.z;
                }

                if let Some(new_rot) = self.rotation {
                    rot.s = new_rot.s;
                    rot.xi = new_rot.v.x;
                    rot.yj = new_rot.v.y;
                    rot.zk = new_rot.v.z;
                }
            }
        }
    }
}