
use crate::game::player::Player;
use crate::systems::chunk::ChunkSystem;
use crate::systems::input::InputSystem;
use crate::systems::physics::PhysicsObject;
use bevy::prelude::*;
use nalgebra::Vector3;
use rc_shared::block::BlockStates;
use crate::systems::camera::MainCamera;
use crate::systems::debugging::DebuggingInfo;
use crate::systems::ui::console::ConsoleData;

const MOVEMENT_SPEED_POSITION: f32 = 2.4;
const MOVEMENT_SPEED_VELOCITY: f32 = 15.0;

pub fn update_input_movement(
    service: Res<InputSystem>,
    mut player: Query<(&mut PhysicsObject, &Player)>,
    camera: Query<&Transform, With<MainCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    chunks: Res<ChunkSystem>,
    block_states: Res<BlockStates>,
    debugging: Res<DebuggingInfo>,
    console_data: Res<ConsoleData>
) {
    if !service.captured || debugging.freecam || console_data.capturing {
        return;
    }

    let camera = camera.single();

    let Ok((mut player_physics, player)) = player.get_single_mut() else {
        println!("NO PLAYER");
        return;
    };

    let forward = camera.forward();
    let forward = Vector3::new(forward.x, 0.0, forward.z);
    let right = forward.cross(&Vector3::new(0.0, 1.0, 0.0));

    let flying_multiplier = if player_physics.touching_ground {
        1.0
    } else {
        0.4
    };

    let sprinting_multiplier = if player.is_sprinting && player_physics.touching_ground {
        1.5
    } else {
        1.0
    };

    let mut proposed_delta = Vector3::zeros();

    if keys.pressed(KeyCode::Space) && player_physics.touching_ground {
        player_physics.velocity.y = 9.0;
    }
    if keys.pressed(KeyCode::KeyW) {
        // W is being held down
        proposed_delta += forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        // S is being held down
        proposed_delta -= forward;
    }
    if keys.pressed(KeyCode::KeyA) {
        // A is being held down
        proposed_delta -= right;
    }
    if keys.pressed(KeyCode::KeyD) {
        // D is being held down
        proposed_delta += right;
    }

    // No change
    if proposed_delta == Vector3::new(0., 0., 0.) {
        return
    }

    proposed_delta = proposed_delta.normalize();

    player_physics.velocity += proposed_delta * MOVEMENT_SPEED_VELOCITY * time.delta_seconds() * flying_multiplier * sprinting_multiplier;
    proposed_delta *= MOVEMENT_SPEED_POSITION * time.delta_seconds() * flying_multiplier * sprinting_multiplier;

    player_physics.translate_with_collision_detection(proposed_delta, &chunks, &block_states);
}
