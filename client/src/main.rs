pub mod error;
pub mod game;
pub mod helpers;
pub mod state;
pub mod systems;

use crate::game::blocks::BlockStatesPlugin;
use crate::game::interaction::mouse_interaction;
use crate::game::inventory::InventoryPlugin;
use crate::game::item::states::ItemStates;
use crate::game::world::WorldPlugin;
use crate::state::AppState;
use crate::systems::asset::atlas::resource_packs::ResourcePacks;
use crate::systems::asset::atlas::ResourcePackData;
use crate::systems::asset::material::chunk::ChunkMaterial;
use crate::systems::asset::parsing::json::JsonAssetLoader;
use crate::systems::asset::parsing::pack::ResourcePackAssetLoader;
use crate::systems::asset::AssetPlugin;
use crate::systems::camera::CameraPlugin;
use crate::systems::chunk::ChunkPlugin;
use crate::systems::input::InputPlugin;
use crate::systems::networking::NetworkingPlugin;
use crate::systems::physics::PhysicsPlugin;
use crate::systems::ui::UIPlugin;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::WindowResizeConstraints;
use bevy_inspector_egui::WorldInspectorPlugin;
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
                        max_width: 1920.0*8.0,
                        max_height: 1080.0*8.0,
                    },
                    ..default()
                },
                ..default()
            }))
        .add_plugin(WorldInspectorPlugin::new())
        
        // add the app state 
        .add_state(AppState::Preloading)
        
        .add_plugin(DebugLinesPlugin::default())
        
        .insert_resource(Msaa { samples: 1 })

        // Networking
        .add_plugin(NetworkingPlugin)
        
        // Interaction
        .add_system(mouse_interaction)
        
        // Chunk loading.rs
        .add_plugin(ChunkPlugin)

        .add_plugin(InputPlugin)

        .add_plugin(CameraPlugin)

        .add_plugin(PhysicsPlugin)

        .add_plugin(WorldPlugin)

        .add_plugin(UIPlugin)

        .add_plugin(InventoryPlugin)
        
        .insert_resource(ItemStates::new())
        
        // Asset Loaders
        .add_asset::<ResourcePacks>()
        .add_asset::<ResourcePackData>()
        .add_plugin(MaterialPlugin::<ChunkMaterial>::default())
        .init_asset_loader::<JsonAssetLoader<ResourcePacks>>()
        .init_asset_loader::<ResourcePackAssetLoader>()

        .add_plugin(BlockStatesPlugin)
        
        // Asset loading.rs
        .add_plugin(AssetPlugin)
        .run();
}
