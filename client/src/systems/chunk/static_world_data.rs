use std::fs;
use bevy::asset::Asset;
use bevy::input::ButtonInput;
use bevy::prelude::{info, KeyCode, Query, Res, ResMut, Transform, TypePath, warn, With};
use nalgebra::{Vector3, Vector4};
use serde::{Deserialize, Serialize};
use rc_shared::chunk::RawChunkData;
use rc_shared::helpers::{from_bevy_vec3, global_to_local_position};
use crate::game::player::Player;
use crate::systems::chunk::ChunkSystem;

/// Provides a means to store static world data in the client.
/// This is used for the Main Menu Screen to show a world.

#[derive(Asset, Debug, Clone, Deserialize, Serialize, TypePath)]
pub struct StaticWorldChunk {
    pub data: RawChunkData,
    pub position: Vector3<i32>
}

#[derive(Asset, Debug, Clone, Deserialize, Serialize, TypePath, Default)]
pub struct StaticWorldData {
    pub data: Vec<StaticWorldChunk>,
    pub camera_position: Vector3<f32>,
    pub camera_rotation: Vector4<f32>
}

/// Saves your surroundings centering on you as static world data
pub fn save_surroundings_system(
    mut chunk_system: ResMut<ChunkSystem>,
    query: Query<&Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if !keys.just_pressed(KeyCode::F4) {
        return
    }

    save_surroundings(&mut chunk_system, query, 2);
}
pub fn save_surroundings(
    chunk_system: &mut ChunkSystem,
    query: Query<&Transform, With<Player>>,
    save_radius: i32
) {

    // Get player pos
    let transform = query.get_single().unwrap();
    let (chunk_pos, _local_pos) = global_to_local_position(
        Vector3::new(transform.translation.x as i32, transform.translation.y as i32, transform.translation.z as i32)
    );

    let mut serialized_data = StaticWorldData {
        data: vec![],
        camera_position: from_bevy_vec3(transform.translation) - (chunk_pos.cast::<f32>() * 16.0),
        camera_rotation: Vector4::new(transform.rotation.x, transform.rotation.y, transform.rotation.z, transform.rotation.w),
    };

    // Get surrounding chunks
    for x in (chunk_pos.x - save_radius)..(chunk_pos.x + save_radius) {
        for y in (chunk_pos.y - save_radius)..(chunk_pos.y + save_radius) {
            for z in (chunk_pos.z - save_radius)..(chunk_pos.z + save_radius) {
                if let Some(chunk_data) = chunk_system.chunks.get(&Vector3::new(x, y, z)) {
                    serialized_data.data.push(StaticWorldChunk {
                        data: chunk_data.world.clone(),
                        position: chunk_data.position - chunk_pos,
                    })
                }
            }
        }
    }

    let save = fs::write("static_world_data.mpk", rmp_serde::to_vec(&serialized_data).unwrap());

    if let Err(e) = save {
        warn!("Failed to save static_world_data.mpk. Reason: {}", e);
    } else {
        info!("Saved static world data");
    }

}