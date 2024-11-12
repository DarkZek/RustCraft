use bevy::input::ButtonInput;
use bevy::log::Level;
use bevy::prelude::{EventWriter, KeyCode, Query, Res, ResMut, Time, Transform, With};
use crate::systems::camera::MainCamera;
use crate::systems::debugging::DebuggingInfo;
use crate::systems::ui::console::{ConsoleData, ConsoleLog};

static MOVEMENT_SPEED: f32 = 10.0;
static BOOST_MOVEMENT_SPEED_MULTIPLIER: f32 = 10.0;

pub fn freecam_activation(
    keys: Res<ButtonInput<KeyCode>>,
    mut freecam: ResMut<DebuggingInfo>,
    mut log: EventWriter<ConsoleLog>
) {
    if !keys.just_pressed(KeyCode::F5) {
        return;
    }

    freecam.freecam = !freecam.freecam;

    log.send(ConsoleLog(format!("Freecam: {}", freecam.freecam), Level::INFO));
}

pub fn freecam_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<MainCamera>>,
    debugging: Res<DebuggingInfo>,
    time: Res<Time>,
    console_data: Res<ConsoleData>
) {

    if !debugging.freecam {
        return
    }
    if console_data.capturing {
        return
    }

    let mut transform = query.single_mut();
    let mut forward = transform.forward().as_vec3() * time.delta_seconds() * MOVEMENT_SPEED;
    let mut left = transform.left().as_vec3() * time.delta_seconds() * MOVEMENT_SPEED;

    if keys.pressed(KeyCode::ShiftLeft) {
        forward *= BOOST_MOVEMENT_SPEED_MULTIPLIER;
        left *= BOOST_MOVEMENT_SPEED_MULTIPLIER;
    }

    if keys.pressed(KeyCode::KeyW) {
        transform.translation += forward;
    }

    if keys.pressed(KeyCode::KeyS) {
        transform.translation -= forward;
    }

    if keys.pressed(KeyCode::KeyA) {
        transform.translation += left;
    }

    if keys.pressed(KeyCode::KeyD) {
        transform.translation -= left;
    }

}