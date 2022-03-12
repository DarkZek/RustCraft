use crate::game::physics::collider::BoxCollider;
use crate::game::physics::PhysicsObject;
use crate::render::camera::Camera;
use nalgebra::{Point3, Vector3};
use specs::{Component, FlaggedStorage, ReadStorage, System, Write};

/// Stores info about the current local player.
pub struct Player {
    pub rot: [f32; 2],
}

impl Player {
    pub fn new() -> Player {
        Player { rot: [0.0, 0.0] }
    }

    pub fn calculate_collider() -> BoxCollider {
        let min = Point3::new(-0.3, 0.0, -0.3);
        let max = Point3::new(0.3, 2.0, 0.3);

        BoxCollider { min, max }
    }
}

pub fn move_forwards(axis: &[i32; 2], side_yaw: f32) -> [f32; 3] {
    let mut pos = [0.0; 3];
    let movement_speed = 0.2;

    let sideways = axis[1] as f32 * movement_speed as f32;
    let forwards = axis[0] as f32 * movement_speed as f32;

    let forwards_yaw = (side_yaw + (0.5 * std::f32::consts::PI)) % (2.0 * std::f32::consts::PI);

    // Move forwards
    pos[0] -= forwards * forwards_yaw.cos();
    pos[2] -= forwards * forwards_yaw.sin();

    // Move side to side
    pos[0] -= sideways * side_yaw.cos();
    pos[2] -= sideways * side_yaw.sin();

    pos
}

#[derive(Debug)]
pub struct PlayerEntity;

impl PlayerEntity {
    pub fn create_physics_object() -> PhysicsObject {
        PhysicsObject {
            velocity: Vector3::new(0.0, 0.0, 0.0),
            position: Vector3::new(0.0, 80.0, 0.0),
            old_position: Vector3::new(0.0, 80.0, 0.0),
            new_position: Vector3::new(0.0, 80.0, 0.0),
            collider: Player::calculate_collider(),
            touching_ground: false,
        }
    }
}

impl Component for PlayerEntity {
    type Storage = FlaggedStorage<Self>;
}

pub struct PlayerEntityCameraSyncSystem;

impl<'a> System<'a> for PlayerEntityCameraSyncSystem {
    type SystemData = (
        ReadStorage<'a, PlayerEntity>,
        ReadStorage<'a, PhysicsObject>,
        Write<'a, Camera>,
    );

    fn run(&mut self, (player, player_physics, mut camera): Self::SystemData) {
        use specs::Join;

        let (_, player_physics) = (&player, &player_physics).join().last().unwrap();

        camera.move_first_person(&Point3::from(player_physics.position));
    }
}
