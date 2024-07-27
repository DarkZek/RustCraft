use std::fs;
use bevy::asset::{Asset, Assets, AssetServer};
use bevy::input::ButtonInput;
use bevy::prelude::{AssetEvent, Camera3d, Commands, EventReader, EventWriter, Handle, info, KeyCode, Mesh, Quat, Query, Res, ResMut, Resource, Transform, TypePath, With};
use nalgebra::{Vector3, Vector4};
use serde::{Deserialize, Serialize};
use rc_shared::block::deserialisation::{BlockStatesFile, DeserialisedBlock};
use rc_shared::chunk::RawChunkData;
use rc_shared::helpers::{from_bevy_vec3, global_to_local_position, to_bevy_vec3};
use crate::game::player::Player;
use crate::systems::asset::AssetService;
use crate::systems::chunk::builder::{RerenderChunkFlag, RerenderChunkFlagContext};
use crate::systems::chunk::ChunkSystem;
use crate::systems::chunk::data::ChunkData;

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

    save_surroundings(&mut chunk_system, query);
}
pub fn save_surroundings(
    mut chunk_system: &mut ChunkSystem,
    query: Query<&Transform, With<Player>>
) {

    // Get player pos
    let transform = query.get_single().unwrap();
    let (chunk_pos, local_pos) = global_to_local_position(
        Vector3::new(transform.translation.x as i32, transform.translation.y as i32, transform.translation.z as i32)
    );

    let mut serialized_data = StaticWorldData {
        data: vec![],
        camera_position: from_bevy_vec3(transform.translation) - (chunk_pos.cast::<f32>() * 16.0),
        camera_rotation: Vector4::new(transform.rotation.x, transform.rotation.y, transform.rotation.z, transform.rotation.w),
    };

    let DIST = 2;
    // Get surrounding chunks
    for x in (chunk_pos.x - DIST)..(chunk_pos.x + DIST) {
        for y in (chunk_pos.y - DIST)..(chunk_pos.y + DIST) {
            for z in (chunk_pos.z - DIST)..(chunk_pos.z + DIST) {
                if let Some(chunk_data) = chunk_system.chunks.get(&Vector3::new(x, y, z)) {
                    serialized_data.data.push(StaticWorldChunk {
                        data: chunk_data.world.clone(),
                        position: chunk_data.position - chunk_pos,
                    })
                }
            }
        }
    }

    fs::write("../../../chunk_lighting_benchmark.mpk", rmp_serde::to_vec(&serialized_data).unwrap()).unwrap();

    info!("Saved static world data");

}