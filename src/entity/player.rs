use crate::game::physics::collider::BoxCollider;
use crate::game::physics::PhysicsObject;
use crate::render::camera::Camera;
use nalgebra::{Point3, Vector3};
use specs::{Component, FlaggedStorage, ReadStorage, System, Write, WriteStorage};

/// Stores info about the current local player.
pub struct Player {
    pub rot: [f32; 2],
}

impl Player {
    pub fn new() -> Player {
        Player { rot: [0.0, 0.0] }
    }

    pub fn calculate_collider(center: Vector3<f32>) -> BoxCollider {
        // Support negative chunks, % on negative numbers returns negative numbers
        let relative_x = ((center.x % 16.0) + 16.0) % 16.0;
        let relative_y = ((center.y % 16.0) + 16.0) % 16.0;
        let relative_z = ((center.z % 16.0) + 16.0) % 16.0;

        let p1 = Point3::new(relative_x - 0.3, relative_y, relative_z - 0.3);
        let p2 = Point3::new(relative_x + 0.3, relative_y + 1.0, relative_z + 0.3);
        BoxCollider {
            p1,
            p2,
            center: center.into(),
        }
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

pub struct PlayerEntityColliderGeneratingSystem;

impl<'a> System<'a> for PlayerEntityColliderGeneratingSystem {
    type SystemData = (
        ReadStorage<'a, PlayerEntity>,
        WriteStorage<'a, PhysicsObject>,
    );

    fn run(&mut self, (player, mut player_physics): Self::SystemData) {
        use specs::Join;

        let (_, player_physics) = (&player, &mut player_physics).join().last().unwrap();

        if player_physics.collider.center != player_physics.position.into() {
            player_physics.collider = Player::calculate_collider(player_physics.position);
        }
    }
}
