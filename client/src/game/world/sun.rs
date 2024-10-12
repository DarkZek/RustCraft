use bevy::prelude::*;

use std::f32::consts::PI;
use web_time::{SystemTime, UNIX_EPOCH};
use crate::systems::asset::AssetService;
use crate::systems::asset::material::chunk_extension::ChunkMaterial;
use crate::systems::asset::material::translucent_chunk_extension::TranslucentChunkMaterial;

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

pub fn update_sun(
    sundata: ResMut<SunData>,
    mut query: Query<&mut Transform>,
    asset_service: Res<AssetService>,
    mut chunk_material: ResMut<Assets<ChunkMaterial>>,
    mut translucent_chunk_material: ResMut<Assets<TranslucentChunkMaterial>>,
) {
    let day_len_ms = 1000 * 60;

    let sun_distance = 600.0;

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        % day_len_ms;

    let day_progress = time as f32 / day_len_ms as f32;

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

    // transform.rotation = Quat::from_axis_angle(
    //     Vec3::new(1.0, 0.0, 0.0),
    //     (day_progress * -PI * 2.0) + (PI / 2.0) + PI,
    // );

    // Update directional light
    // let rot = Quat::from_axis_angle(
    //     Vec3::new(1.0, 0.0, 0.0),
    //     (day_progress * -PI * 2.0) + (PI / 2.0) + PI,
    // );

    //let mut transform = query.get_mut(sundata.directional_light).unwrap();
    //transform.rotation = rot;

    // Set sunlight strength
    chunk_material.get_mut(&asset_service.opaque_texture_atlas_material).unwrap().extension.uniform.sunlight_strength = (day_progress * std::f32::consts::PI).sin();
    translucent_chunk_material.get_mut(&asset_service.translucent_texture_atlas_material).unwrap().extension.uniform.sunlight_strength = (day_progress * std::f32::consts::PI).sin();
}
