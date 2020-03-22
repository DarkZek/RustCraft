use cgmath::{Vector3};
use crate::game::physics::{PhysicsObject, Hitbox};

pub struct Player {
    pub pos: Vector3<f32>,
    pub rot: [f32; 2],
    pub velocity: Vector3<f32>
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vector3 {x: 10.0, y: 50.0, z: 10.0 },
            rot: [0.0, 0.0],
            velocity: Vector3 {x: 0.0, y: 0.0, z: 0.0 },
        }
    }

    pub fn move_forwards(&mut self, axis: &[i32; 2], delta_time: f64) {
        let pos = &mut self.pos;
        let movement_speed = 20.0;

        let sideways = axis[1] as f32 * movement_speed as f32 * delta_time as f32;
        let forwards = axis[0] as f32 * movement_speed as f32 * delta_time as f32;

        let side_yaw = self.rot[0];
        let forwards_yaw = (side_yaw + (0.5 * std::f32::consts::PI)) % (2.0 * std::f32::consts::PI);

        // Move forwards
        pos[0] -= forwards * forwards_yaw.cos();
        pos[2] -= forwards * forwards_yaw.sin();

        // Move side to side
        pos[0] -= sideways * side_yaw.cos();
        pos[2] -= sideways * side_yaw.sin();
    }
}

impl PhysicsObject for Player {
    fn get_hitbox(&self) -> Hitbox {
        unimplemented!()
    }

    fn get_velocity(&self) -> Vector3<f32> {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: Vector3<f32>) {
        self.velocity = velocity;
    }

    fn translate(&mut self, direction: Vector3<f32>) {
        self.pos += direction;
    }
}