use std::time::{Duration, Instant};
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Local, Projection, Query, Res};
use crate::game::player::Player;

const MAX_SPRINT_TAP_GAP: Duration = Duration::from_millis(500);

pub struct SprintMovementData {
    last_sprint_time: Instant
}

impl Default for SprintMovementData {
    fn default() -> Self {
        SprintMovementData {
            last_sprint_time: Instant::now()
        }
    }
}

// Allow users to start sprinting when they double tap w
pub fn detect_sprinting(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Player)>,
    mut local: Local<SprintMovementData>,
    mut projection: Query<(&mut Projection)>,
) {

    let mut player = player.single_mut();
    let mut projection = projection.single_mut();

    if keys.just_released(KeyCode::KeyW) {
        set_sprinting(false, &mut player, &mut projection);
    }

    if keys.just_pressed(KeyCode::KeyW) {

        if local.last_sprint_time.elapsed() < MAX_SPRINT_TAP_GAP {
            // Start sprinting
            set_sprinting(true, &mut player, &mut projection);
        }

        local.last_sprint_time = Instant::now();
    }

    if keys.just_pressed(KeyCode::ControlLeft) && keys.pressed(KeyCode::KeyW) {
        set_sprinting(true, &mut player, &mut projection);
    }
}

fn set_sprinting(sprinting: bool, player: &mut Player, projection: &mut Projection) {
    player.is_sprinting = sprinting;
    if let Projection::Perspective(projection) = projection {
        if sprinting {
            projection.fov = std::f32::consts::PI / 2.8;
        } else {
            projection.fov = std::f32::consts::PI / 3.0;
        }
    }
}