use bevy::color::palettes::css::MAGENTA;
use bevy::prelude::{Gizmos, Query, Transform, Vec3, With};
use nalgebra::Vector3;
use rc_shared::CHUNK_SIZE;
use rc_shared::helpers::{global_to_local_position, to_bevy_vec3};
use crate::systems::camera::MainCamera;

const DISPLAY_DISTANCE: i32 = 5;

/// Get the height of the skylight heightmap of the chunk at the camera and draw cubes to show it
pub fn draw_chunk_boundary(
    camera: Query<&Transform, With<MainCamera>>,
    mut gizmos: Gizmos
) {
    let pos = camera.single();

    let (chunk_pos, _) = global_to_local_position(Vector3::new(pos.translation.x as i32, pos.translation.y as i32, pos.translation.z as i32));

    let chunk_center = (chunk_pos.cast::<f32>() * CHUNK_SIZE as f32)
        + (Vector3::new(CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE).cast::<f32>() / 2.0);

    gizmos.cuboid(
        Transform::from_translation(to_bevy_vec3(chunk_center)).with_scale(Vec3::splat(CHUNK_SIZE as f32)),
        MAGENTA,
    );
}