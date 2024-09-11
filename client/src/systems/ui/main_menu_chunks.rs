use bevy::asset::{AssetEvent, Assets, AssetServer, Handle};
use bevy::math::Quat;
use bevy::prelude::{Camera, Commands, EventReader, EventWriter, Mesh, Query, Res, ResMut, Resource, Transform, With};
use nalgebra::Vector3;
use rc_shared::helpers::to_bevy_vec3;
use crate::systems::asset::AssetService;
use crate::systems::chunk::builder::{RerenderChunkRequest, RerenderChunkFlagContext};
use crate::systems::chunk::ChunkSystem;
use crate::systems::chunk::static_world_data::StaticWorldData;

#[derive(Resource, Default)]
pub struct MainMenuWorldState {
    asset: Option<Handle<StaticWorldData>>
}

pub fn load_main_menu_world(asset_server: Res<AssetServer>, mut data: ResMut<MainMenuWorldState>) {
    data.asset = Some(asset_server.load("main_menu_world.mpk").into());
}

pub fn handle_loaded_main_menu_world(
    mut events: EventReader<AssetEvent<StaticWorldData>>,
    assets: ResMut<Assets<StaticWorldData>>,
    mut chunk_system: ResMut<ChunkSystem>,
    mut commands: Commands,
    asset_service: Res<AssetService>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut data: ResMut<MainMenuWorldState>,
    mut rerender_chunks: EventWriter<RerenderChunkRequest>,
    mut query: Query<&mut Transform, With<Camera>>
) {

    for event in events.read() {

        match event {
            AssetEvent::Added { .. } => {}
            _ => return
        }

        data.asset = None;

        let (_, loaded_data) = assets.iter().next().unwrap();

        for chunk in loaded_data.data.clone() {
            rerender_chunks.send(RerenderChunkRequest {
                chunk: chunk.position,
                context: RerenderChunkFlagContext::None,
            });

            chunk_system.create_chunk(chunk.position, chunk.data, &mut commands, &asset_service, &mut meshes);
        }

        // Set camera position
        let mut camera_transform = query.get_single_mut().unwrap();

        camera_transform.translation = to_bevy_vec3(loaded_data.camera_position);
        camera_transform.rotation = Quat::from_array([
            loaded_data.camera_rotation.x,
            loaded_data.camera_rotation.y,
            loaded_data.camera_rotation.z,
            loaded_data.camera_rotation.w
        ]);
    }
}

pub fn remove_main_menu_world(
    mut chunk_system: ResMut<ChunkSystem>,
    mut commands: Commands,) {
    let positions = chunk_system.chunks.keys().cloned().collect::<Vec<Vector3<i32>>>();
    for pos in positions {
        chunk_system.unload_chunk(pos, &mut commands);
    }
}