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

#[derive(Resource, Default)]
pub struct MainMenuWorldState {
    asset: Option<Handle<StaticWorldData>>
}

pub fn load_main_menu_world(asset_server: Res<AssetServer>, mut data: ResMut<MainMenuWorldState>) {
    data.asset = Some(asset_server.load("main_menu_world.mpk").into());
}

pub fn handle_loaded_static_world(
    mut events: EventReader<AssetEvent<StaticWorldData>>,
    assets: ResMut<Assets<StaticWorldData>>,
    mut chunk_system: ResMut<ChunkSystem>,
    mut commands: Commands,
    asset_service: Res<AssetService>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut data: ResMut<MainMenuWorldState>,
    mut rerender_chunks: EventWriter<RerenderChunkFlag>,
    mut query: Query<&mut Transform, With<Player>>
) {

    for event in events.read() {

        match event {
            AssetEvent::Added { .. } => {}
            _ => return
        }

        data.asset = None;

        let (_, loaded_data) = assets.iter().next().unwrap();

        for chunk in loaded_data.data.clone() {
            rerender_chunks.send(RerenderChunkFlag {
                chunk: chunk.position,
                context: RerenderChunkFlagContext::None,
            });

            chunk_system.create_chunk(chunk.position, chunk.data, &mut commands, &asset_service, &mut meshes);
        }

        // Set player position
        let mut player_transform = query.get_single_mut().unwrap();

        player_transform.translation = to_bevy_vec3(loaded_data.camera_position);
        player_transform.rotation = Quat::from_array([
            loaded_data.camera_rotation.x,
            loaded_data.camera_rotation.y,
            loaded_data.camera_rotation.z,
            loaded_data.camera_rotation.w
        ]);
    }
}

pub fn remove_static_world(
    mut chunk_system: ResMut<ChunkSystem>,
    mut commands: Commands,) {
    let positions = chunk_system.chunks.keys().cloned().collect::<Vec<Vector3<i32>>>();
    for pos in positions {
        chunk_system.unload_chunk(pos, &mut commands);
    }
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

    fs::write("../assets/main_menu_world.mpk", rmp_serde::to_vec(&serialized_data).unwrap()).unwrap();

    info!("Saved static world data");

}