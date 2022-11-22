pub mod error;
pub mod game;
pub mod helpers;
pub mod render;
pub mod services;
pub mod state;

use crate::game::blocks::create_block_states;
use crate::game::interaction::mouse_interaction;
use crate::game::parsing::json::JsonAssetLoader;
use crate::game::parsing::pack::ResourcePackAssetLoader;
use crate::services::asset::atlas::resource_packs::ResourcePacks;
use crate::services::asset::atlas::{
    build_texture_atlas, load_resource_zips, AtlasLoadingStage, ResourcePackData,
};
use crate::services::asset::create_asset_service;
use crate::services::asset::material::chunk::ChunkMaterial;
use crate::services::camera::CameraPlugin;

use crate::services::chunk::systems::mesh_builder::mesh_builder;
use crate::services::chunk::ChunkPlugin;
use crate::services::input::InputPlugin;
use crate::services::networking::NetworkingPlugin;
use crate::services::physics::PhysicsPlugin;
use crate::services::ui::UIPlugin;

use crate::game::blocks::loader::{track_blockstate_changes, BlockStateAssetLoader};
use crate::game::blocks::loading::BlockStatesFile;
use crate::game::blocks::states::BlockStates;
use crate::state::AppState;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;

use crate::game::inventory::InventoryPlugin;
use crate::game::item::states::ItemStates;
use bevy::window::WindowResizeConstraints;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;

#[rustfmt::skip]
fn main() {
    
    App::new()
        // add the app state type
        .add_state(AppState::Loading)
        .add_plugins(
            DefaultPlugins
            .set(LogPlugin {
                filter: "wgpu=error,rustcraft=debug,naga=error,bevy_app=info".into(),
                level: Level::DEBUG,
            })
            .set(AssetPlugin {
                watch_for_changes: true,
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                window: WindowDescriptor {
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
                },
                ..default()
            }))
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(Msaa { samples: 4 })

        // Networking
        .add_plugin(NetworkingPlugin)
        
        // Interaction
        .add_system(mouse_interaction)
        
        // Chunk loading
        .add_plugin(ChunkPlugin)

        .add_plugin(InputPlugin)

        .add_plugin(CameraPlugin)

        .add_plugin(PhysicsPlugin)
        
        .add_plugin(UIPlugin)

        .add_plugin(InventoryPlugin)
        
        .insert_resource(ItemStates::new())
        
        // Asset Loaders
        .add_asset::<ResourcePacks>()
        .add_asset::<ResourcePackData>()
        .add_plugin(MaterialPlugin::<ChunkMaterial>::default())
        .init_asset_loader::<JsonAssetLoader<ResourcePacks>>()
        .init_asset_loader::<ResourcePackAssetLoader>()

        .add_asset::<BlockStatesFile>()
        .init_asset_loader::<BlockStateAssetLoader>()
        .add_startup_system(create_block_states)
        .insert_resource(BlockStates::new())
        .add_system(track_blockstate_changes)
        
        .add_system(mesh_builder)
        
        // Asset loading
        .insert_resource(AtlasLoadingStage::AwaitingIndex)
        .add_startup_system(create_asset_service)
        .add_system(load_resource_zips)
        .add_system(build_texture_atlas)
        .run();
}
