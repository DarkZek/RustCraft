use bevy::color::palettes::basic::YELLOW;
use bevy::prelude::{Gizmos, Query, Res, Transform, Vec3, With};
use nalgebra::{Vector2, Vector3};
use rc_shared::CHUNK_SIZE;
use rc_shared::helpers::{local_to_global_position, to_bevy_vec3};
use crate::systems::camera::MainCamera;
use crate::systems::chunk::ChunkSystem;

const DISPLAY_DISTANCE: i32 = 5;

/// Get the height of the skylight heightmap of the chunk at the camera and draw cubes to show it
pub fn draw_skylight(
    camera: Query<&Transform, With<MainCamera>>,
    chunk_data: Res<ChunkSystem>,
    mut gizmos: Gizmos
) {
    let camera = camera.single();

    let x_chunk = (camera.translation.x / CHUNK_SIZE as f32).floor() as i32;
    let z_chunk = (camera.translation.z / CHUNK_SIZE as f32).floor() as i32;

    let Some(column) = chunk_data.chunk_columns.get(&Vector2::new(x_chunk, z_chunk)) else {
        return
    };

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let Some(y_level) = column.skylight_level[x][z] else {
                continue
            };

            let mut pos = Vector3::new(
                (x_chunk as f32 * CHUNK_SIZE as f32) + x as f32,
                y_level as f32,
                (z_chunk as f32 * CHUNK_SIZE as f32) + z as f32,
            );

            // Offset to be in center of block
            pos += Vector3::new(0.5, 0.5, 0.5);

            gizmos.cuboid(
                Transform::from_translation(to_bevy_vec3(pos)).with_scale(Vec3::splat(0.25)),
                YELLOW,
            );
        }
    }
}