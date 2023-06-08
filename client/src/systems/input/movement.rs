use crate::game::player::Player;
use crate::systems::input::InputSystem;
use crate::systems::physics::PhysicsObject;
use bevy::prelude::*;
use nalgebra::Vector3;

pub fn update_input_movement(
    service: Res<InputSystem>,
    mut player: Query<(&mut PhysicsObject, &Player)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if !service.captured {
        return;
    }

    let (mut player_physics, player) = player.single_mut();

    let forward = -Vector3::new(player.yaw.sin(), 0.0, player.yaw.cos());

    let right = forward.cross(&Vector3::new(0.0, 1.0, 0.0));

    if keys.pressed(KeyCode::Space) {
        player_physics.position.y += 0.2;
    }
    if keys.pressed(KeyCode::LShift) {
        //player_physics.position.y -= 0.2;
    }
    if keys.pressed(KeyCode::W) {
        // W is being held down
        //player_physics.position += forward * 0.02;
        player_physics.velocity += forward * 2.1 * time.delta_seconds() * 50.0;
    }
    if keys.pressed(KeyCode::S) {
        // W is being held down
        //player_physics.position -= forward * 0.02;
        player_physics.velocity -= forward * 2.1 * time.delta_seconds() * 50.0;
    }
    if keys.pressed(KeyCode::A) {
        // W is being held down
        //player_physics.position -= right * 0.02;
        player_physics.velocity -= right * 2.1 * time.delta_seconds() * 50.0;
    }
    if keys.pressed(KeyCode::D) {
        // W is being held down
        //player_physics.position += right * 0.02;
        player_physics.velocity += right * 2.1 * time.delta_seconds() * 50.0;
    }
}
