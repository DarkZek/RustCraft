use bevy::prelude::*;

use bevy::pbr::CascadeShadowConfigBuilder;
use std::f32::consts::PI;
use std::time::{SystemTime, UNIX_EPOCH};

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
    assets: Res<AssetServer>,
) {
    // TODO: Spawn this attached to the camera so it moves around with it
    let sun_sprite = commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Rectangle::new(150.0, 150.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                base_color_texture: Some(assets.load("textures/world/sun.png")),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        })
        .with_children(|_c| {})
        .id();

    let moon_sprite = commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Rectangle::new(150.0, 150.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                base_color_texture: Some(assets.load("textures/world/moon.png")),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(PI / 2.0, 0.0, 0.0)),
            ..default()
        })
        .id();

    let directional_light = commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::rgb(1., 1., 1.),
                illuminance: 3000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                rotation: Quat::from_rotation_x(3.),
                ..default()
            },
            cascade_shadow_config: CascadeShadowConfigBuilder {
                first_cascade_far_bound: 4.0,
                maximum_distance: 100.0,
                ..default()
            }
            .into(),
            ..default()
        })
        .id();

    commands.insert_resource(SunData {
        sun_sprite,
        moon_sprite,
        directional_light,
    });
}

pub fn update_sun(sundata: ResMut<SunData>, mut query: Query<&mut Transform>) {
    let day_len_ms = 1000 * 60;

    let sun_distance = 600.0;

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        % day_len_ms;

    let mut day_progress = time as f32 / day_len_ms as f32;

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

    //let mut transform = query.get_mut(sundata.directional_light).unwrap();
    //transform.rotation = rot;
}
