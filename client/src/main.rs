pub mod error;
pub mod game;
pub mod helpers;
pub mod render;
pub mod services;
pub mod state;

use crate::game::blocks::{create_block_states, BlockStatesPlugin};
use crate::game::interaction::mouse_interaction;
use crate::game::parsing::json::JsonAssetLoader;
use crate::game::parsing::pack::ResourcePackAssetLoader;
use crate::services::asset::atlas::resource_packs::ResourcePacks;
use crate::services::asset::atlas::{
    build_texture_atlas, load_resource_zips, AtlasLoadingStage, ResourcePackData,
};
use crate::services::asset::material::chunk::ChunkMaterial;
use crate::services::asset::{create_asset_service, AssetPlugin};
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
use crate::services::ui::loading::{set_loading, LoadingPlugin};
use crate::services::ui::main_menu::MainMenuPlugin;
use bevy::window::WindowResizeConstraints;
use bevy_prototype_debug_lines::DebugLinesPlugin;

#[rustfmt::skip]
fn main() {
    
    App::new()
        .add_plugins(
            DefaultPlugins
            .set(LogPlugin {
                filter: "wgpu=error,rustcraft=debug,naga=error,bevy_app=info".into(),
                level: Level::DEBUG,
            })
            .set(bevy::prelude::AssetPlugin {
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
        
        // add the app state 
        .add_state(AppState::Preloading)
        
        .add_plugin(DebugLinesPlugin::default())
        
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


        .add_plugin(LoadingPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(MainMenuPlugin)

        .add_plugin(InventoryPlugin)
        
        .insert_resource(ItemStates::new())
        
        // Asset Loaders
        .add_asset::<ResourcePacks>()
        .add_asset::<ResourcePackData>()
        .add_plugin(MaterialPlugin::<ChunkMaterial>::default())
        .init_asset_loader::<JsonAssetLoader<ResourcePacks>>()
        .init_asset_loader::<ResourcePackAssetLoader>()

        .add_plugin(BlockStatesPlugin)
        
        .add_system(mesh_builder)
        
        // Asset loading
        .add_plugin(AssetPlugin)
        .run();
}
