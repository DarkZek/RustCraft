use bevy::input::ButtonInput;
use bevy::log::Level;
use bevy::prelude::{EventWriter, info, KeyCode, Query, Res, ResMut, Resource, Time, Transform, With};
use crate::systems::camera::{Freecam, MainCamera};
use crate::systems::ui::console::ConsoleLog;

static MOVEMENT_SPEED: f32 = 10.0;
static BOOST_MOVEMENT_SPEED_MULTIPLIER: f32 = 10.0;

pub fn freecam_activation(
    keys: Res<ButtonInput<KeyCode>>,
    mut freecam: ResMut<Freecam>,
    mut log: EventWriter<ConsoleLog>
) {
    if !keys.just_pressed(KeyCode::F5) {
        return;
    }

    freecam.enabled = !freecam.enabled;

    log.send(ConsoleLog(format!("Freecam: {}", freecam.enabled), Level::INFO));
}

pub fn freecam_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<MainCamera>>,
    freecam: Res<Freecam>,
    time: Res<Time>
) {

    if !freecam.enabled {
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