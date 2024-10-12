use bevy::prelude::*;

use std::f32::consts::PI;
use bevy::color::palettes::basic::BLACK;
use bevy::color::palettes::tailwind::BLUE_300;
use bevy::math::VectorSpace;
use web_time::{SystemTime, UNIX_EPOCH};
use rc_shared::time::daylight_amount;
use crate::systems::asset::AssetService;
use crate::systems::asset::material::chunk_extension::ChunkMaterial;
use crate::systems::asset::material::translucent_chunk_extension::TranslucentChunkMaterial;
use crate::systems::camera::MainCamera;

#[derive(Resource)]
pub struct SunData {
    sun_sprite: Entity,
    moon_sprite: Entity
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

    commands.insert_resource(SunData {
        sun_sprite,
        moon_sprite,
    });
}

const DAY_LEN_MS: u128 = 1000*60;

const SUN_DISTANCE: f32 = 600.0;

pub fn update_sun(
    sundata: ResMut<SunData>,
    mut query: Query<&mut Transform>,
    asset_service: Res<AssetService>,
    mut chunk_material: ResMut<Assets<ChunkMaterial>>,
    mut translucent_chunk_material: ResMut<Assets<TranslucentChunkMaterial>>,
    mut camera: Query<&mut Camera, With<MainCamera>>
) {
    let day_len_ms = 1000 * 60;

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        % day_len_ms;

    // 0 is sunrise, 0.25 is midday, 0.5 is sunset, 0.75 is midnight
    let day_progress = time as f32 / day_len_ms as f32;

    rotate_sun_moon_sprite(
        day_progress + 0.75 - 0.1,
        &mut query,
        &sundata.sun_sprite
    );

    rotate_sun_moon_sprite(
        day_progress + 0.25 - 0.1,
        &mut query,
        &sundata.moon_sprite
    );

    // Update directional light
    // let rot = Quat::from_axis_angle(
    //     Vec3::new(1.0, 0.0, 0.0),
    //     (day_progress * -PI * 2.0) + (PI / 2.0) + PI,
    // );

    //let mut transform = query.get_mut(sundata.directional_light).unwrap();
    //transform.rotation = rot;

    // Set sunlight strength
    let strength = daylight_amount(day_progress);
    let sunlight_strength = (strength + 0.01) * 0.98;
    chunk_material.get_mut(&asset_service.opaque_texture_atlas_material).unwrap().extension.uniform.sunlight_strength = sunlight_strength;
    translucent_chunk_material.get_mut(&asset_service.translucent_texture_atlas_material).unwrap().extension.uniform.sunlight_strength = sunlight_strength;

    camera.single_mut().clear_color = ClearColorConfig::Custom(Color::from(BLACK.lerp(BLUE_300, strength)));
}

fn rotate_sun_moon_sprite(
    day_progress: f32,
    query: &mut Query<&mut Transform>,
    entity: &Entity
) {
    let rotation_amount = day_progress * 2.0 * PI + (PI / 4.0);
    let x = (rotation_amount.cos() - rotation_amount.sin()) * SUN_DISTANCE;
    let y = (rotation_amount.cos() + rotation_amount.sin()) * SUN_DISTANCE;

    let mut transform = query.get_mut(*entity).unwrap();
    transform.translation = Vec3::new(0.0, y, x);

    transform.rotation = Quat::from_axis_angle(
        Vec3::new(1.0, 0.0, 0.0),
        (day_progress * -PI * 2.0) + (PI / 2.0),
    );
}
