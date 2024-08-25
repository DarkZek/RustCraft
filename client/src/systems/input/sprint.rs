use web_time::{Duration, Instant};
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Local, Projection, Query, Res, Time, With};
use rc_shared::helpers::Lerp;
use crate::game::player::Player;
use crate::systems::camera::MainCamera;

const MAX_SPRINT_TAP_GAP: Duration = Duration::from_millis(500);

const SPRINTING_FOV: f32 = std::f32::consts::PI / 2.6;
const WALKING_FOV: f32 = std::f32::consts::PI / 3.0;

pub struct SprintMovementData {
    last_sprint_time: Instant,
    fov_animation: f32
}

impl Default for SprintMovementData {
    fn default() -> Self {
        SprintMovementData {
            last_sprint_time: Instant::now(),
            fov_animation: 0.
        }
    }
}

// Allow users to start sprinting when they double tap w
pub fn detect_sprinting(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Player>,
    mut local: Local<SprintMovementData>,
    mut projection: Query<&mut Projection, With<MainCamera>>,
    time: Res<Time>,
) {

    let Ok(mut player) = player.get_single_mut() else {
        return
    };
    let mut projection = projection.single_mut();

    // Update fov
    let target_animation = if player.is_sprinting { 1.0 } else { 0.0 };
    local.fov_animation = local.fov_animation.lerp(target_animation, 7.5 * time.delta_seconds());

    if let Projection::Perspective(projection) = &mut *projection {
        projection.fov = WALKING_FOV.lerp(SPRINTING_FOV, local.fov_animation);
    }

    if keys.just_released(KeyCode::KeyW) {
        player.is_sprinting = false;
    }

    if keys.just_pressed(KeyCode::KeyW) {

        if local.last_sprint_time.elapsed() < MAX_SPRINT_TAP_GAP {
            // Start sprinting
            player.is_sprinting = true;
        }

        local.last_sprint_time = Instant::now();
    }

    if keys.just_pressed(KeyCode::ControlLeft) && keys.pressed(KeyCode::KeyW) {
        player.is_sprinting = true;
    }
}