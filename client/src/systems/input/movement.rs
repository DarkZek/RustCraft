use crate::game::player::Player;
use crate::systems::chunk::ChunkSystem;
use crate::systems::input::InputSystem;
use crate::systems::physics::PhysicsObject;
use bevy::prelude::*;
use nalgebra::Vector3;
use rc_shared::block::BlockStates;

const MOVEMENT_SPEED_POSITION: f32 = 2.0;
const MOVEMENT_SPEED_VELOCITY: f32 = 15.0;

pub fn update_input_movement(
    service: Res<InputSystem>,
    mut player: Query<(&mut PhysicsObject, &Player)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    chunks: Res<ChunkSystem>,
    block_states: Res<BlockStates>,
) {
    if !service.captured {
        return;
    }

    let (mut player_physics, player): (Mut<PhysicsObject>, &Player) = player.single_mut();

    let forward = -Vector3::new(player.yaw.sin(), 0.0, player.yaw.cos());
    let right = forward.cross(&Vector3::new(0.0, 1.0, 0.0));

    let flying_multiplier = if player_physics.touching_ground {
        1.0
    } else {
        0.4
    };

    let mut proposed_delta = Vector3::zeros();

    if keys.just_pressed(KeyCode::Space) && player_physics.touching_ground {
        player_physics.velocity.y += 12.0;
    }
    if keys.pressed(KeyCode::KeyW) {
        // W is being held down
        proposed_delta +=
            forward * MOVEMENT_SPEED_POSITION * time.delta_seconds() * flying_multiplier;
        player_physics.velocity +=
            forward * time.delta_seconds() * MOVEMENT_SPEED_VELOCITY * flying_multiplier;
    }
    if keys.pressed(KeyCode::KeyS) {
        // W is being held down
        proposed_delta -=
            forward * MOVEMENT_SPEED_POSITION * time.delta_seconds() * flying_multiplier;
        player_physics.velocity -=
            forward * time.delta_seconds() * MOVEMENT_SPEED_VELOCITY * flying_multiplier;
    }
    if keys.pressed(KeyCode::KeyA) {
        // W is being held down
        proposed_delta -=
            right * MOVEMENT_SPEED_POSITION * time.delta_seconds() * flying_multiplier;
        player_physics.velocity -=
            right * time.delta_seconds() * MOVEMENT_SPEED_VELOCITY * flying_multiplier;
    }
    if keys.pressed(KeyCode::KeyD) {
        // W is being held down
        proposed_delta +=
            right * MOVEMENT_SPEED_POSITION * time.delta_seconds() * flying_multiplier;
        player_physics.velocity +=
            right * time.delta_seconds() * MOVEMENT_SPEED_VELOCITY * flying_multiplier;
    }

    player_physics.translate_with_collision_detection(proposed_delta, &chunks, &block_states);
}
