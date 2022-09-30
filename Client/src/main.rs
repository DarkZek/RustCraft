pub mod game;
pub mod helpers;
pub mod render;
pub mod services;

use crate::game::blocks::BlockStates;
use crate::game::interaction::mouse_interaction;
use crate::game::parsing::json::JsonAssetLoader;
use crate::game::parsing::pack::ResourcePackAssetLoader;
use crate::services::asset::atlas::resource_packs::ResourcePacks;
use crate::services::asset::atlas::{
    build_texture_atlas, load_resource_zips, AtlasLoadingStage, ResourcePackData,
};
use crate::services::asset::create_asset_service;
use crate::services::chunk::data::partial::PartialChunks;
use crate::services::chunk::data::{ChunkData, RawChunkData};
use crate::services::chunk::systems::mesh_builder::{mesh_builder, RerenderChunkFlag};
use crate::services::chunk::ChunkPlugin;
use crate::services::networking::NetworkingPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::window::WindowResizeConstraints;
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_testing_protocol::channels::Channels;
use bevy_testing_protocol::config::network_config;
use bevy_testing_protocol::constants::CHUNK_SIZE;
use bevy_testing_protocol::protocol::Protocol;
use nalgebra::Vector3;
use std::fs::File;
use std::io::Write;

#[rustfmt::skip]
fn main() {
    
    App::new()
        .insert_resource(WindowDescriptor {
            title: "app".to_string(),
            width: 1280.,
            height: 720.,
            position: WindowPosition::Automatic,
            resize_constraints: WindowResizeConstraints {
                min_width: 256.0,
                min_height: 256.0,
                max_width: 1920.0,
                max_height: 1080.0,
            },
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(Msaa { samples: 4 })

        // Networking
        .add_plugin(NetworkingPlugin)
        
        // Interaction
        .add_system(mouse_interaction)
        
        // Chunk loading
        .add_plugin(ChunkPlugin)
        
        // Asset Loaders
        .add_asset::<ResourcePacks>()
        .add_asset::<ResourcePackData>()
        .init_asset_loader::<JsonAssetLoader<ResourcePacks>>()
        .init_asset_loader::<ResourcePackAssetLoader>()
         
        .insert_resource(BlockStates::new())
        
        .add_system(mesh_builder)
        
        // Camera
        .add_plugin(PlayerPlugin)
        
        // Asset loading
        .insert_resource(AtlasLoadingStage::AwaitingIndex)
        .add_startup_system(create_asset_service)
        .add_system(load_resource_zips)
        .add_system(build_texture_atlas)
        .run();
}
