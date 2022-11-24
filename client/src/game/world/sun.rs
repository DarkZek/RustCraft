use bevy::prelude::*;
use std::f32::consts::PI;

pub fn setup_sun(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::GREEN,
        brightness: 10000.0,
    });

    // TODO: Spawn this attached to the camera so it moves around with it
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(1., 1., 1.),
            illuminance: 50000.0,
            shadow_projection: OrthographicProjection {
                left: -40.0,
                right: 40.0,
                bottom: -40.0,
                top: 40.0,
                near: -50.0,
                far: 50.0,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            PI / 6.,
            -PI / 5.,
        )),
        ..default()
    });
}
