use crate::game::player::Player;
use bevy::prelude::shape::Quad;
use bevy::prelude::*;
use bevy::render::primitives::Plane;
use bevy::utils::Instant;
use nalgebra::Vector3;
use std::f32::consts::PI;
use std::time::{SystemTime, UNIX_EPOCH};
use zip::DateTime;

#[derive(Resource)]
pub struct SunData {
    sun_sprite: Entity,
    moon_sprite: Entity,
    directional_light: Entity,
}

pub fn setup_sun(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut assets: Res<AssetServer>,
) {
    // TODO: Spawn this attached to the camera so it moves around with it
    let sun_sprite = commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Quad::new(Vec2::new(150.0, 150.0)))),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                base_color_texture: Some(assets.load("textures/world/sun.png")),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        })
        .with_children(|c| {})
        .id();

    let moon_sprite = commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Quad::new(Vec2::new(150.0, 150.0)))),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                base_color_texture: Some(assets.load("textures/world/moon.png")),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(PI / 2.0, 0.0, 0.0)),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        })
        .id();

    let directional_light = commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::rgb(1., 1., 1.),
                illuminance: 50000.0,
                shadow_projection: OrthographicProjection {
                    left: -40.0,
                    right: 40.0,
                    bottom: -40.0,
                    top: 40.0,
                    near: -20.0,
                    far: 50.0,
                    ..default()
                },
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_euler(
                EulerRot::XYZ,
                PI / 2.0,
                0.0,
                0.0,
            )),
            ..default()
        })
        .id();

    commands.insert_resource(SunData {
        sun_sprite,
        moon_sprite,
        directional_light,
    });
}

pub fn update_sun(mut sundata: ResMut<SunData>, mut query: Query<&mut Transform>) {
    let day_len_ms = 1000 * 60;

    let sun_distance = 600.0;

    let mut time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        % day_len_ms;

    let mut day_progress = (time as f32 / day_len_ms as f32);

    day_progress = 0.05;

    let rotation_amount = day_progress * 2.0 * PI + (PI / 4.0);
    let x = (rotation_amount.cos() - rotation_amount.sin()) * sun_distance;
    let y = (rotation_amount.cos() + rotation_amount.sin()) * sun_distance;

    let mut transform = query.get_mut(sundata.sun_sprite).unwrap();
    transform.translation = Vec3::new(0.0, y, x);

    transform.rotation = Quat::from_axis_angle(
        Vec3::new(1.0, 0.0, 0.0),
        (day_progress * -PI * 2.0) + (PI / 2.0),
    );

    let rotation_amount = day_progress * 2.0 * PI + (PI / 4.0) + PI;
    let x = (rotation_amount.cos() - rotation_amount.sin()) * sun_distance;
    let y = (rotation_amount.cos() + rotation_amount.sin()) * sun_distance;

    let mut transform = query.get_mut(sundata.moon_sprite).unwrap();
    transform.translation = Vec3::new(0.0, y, x);

    transform.rotation = Quat::from_axis_angle(
        Vec3::new(1.0, 0.0, 0.0),
        (day_progress * -PI * 2.0) + (PI / 2.0) + PI,
    );

    // Update directional light
    let rot = Quat::from_axis_angle(
        Vec3::new(1.0, 0.0, 0.0),
        (day_progress * -PI * 2.0) + (PI / 2.0) + PI,
    );

    let mut transform = query.get_mut(sundata.directional_light).unwrap();
    transform.rotation = rot;
}
