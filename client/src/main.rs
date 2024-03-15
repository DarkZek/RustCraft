pub mod game;
pub mod state;
pub mod systems;

use crate::game::events::GameEventsPlugin;
use crate::game::interaction::highlight::{
    mouse_highlight_interaction, setup_highlights, HighlightData,
};
use crate::game::interaction::mouse_interaction;
use crate::game::inventory::InventoryPlugin;
use crate::game::state::{create_states, track_blockstate_changes, track_itemstate_changes};
use crate::game::world::WorldPlugin;
use crate::state::AppState;
use crate::systems::asset::atlas::atlas::TEXTURE_ATLAS;
use crate::systems::asset::atlas::resource_packs::{ResourcePackData, ResourcePacks};
use crate::systems::asset::material::chunk::ChunkMaterial;
use crate::systems::asset::parsing::json::JsonAssetLoader;
use crate::systems::asset::parsing::pack::ResourcePackAssetLoader;
use crate::systems::asset::AssetPlugin;
use crate::systems::camera::CameraPlugin;
use crate::systems::chunk::ChunkPlugin;
use crate::systems::input::InputPlugin;
use crate::systems::networking::ClientNetworkingPlugin;
use crate::systems::physics::PhysicsPlugin;
use crate::systems::ui::UIPlugin;
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasPlugin;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_polyline::PolylinePlugin;
use rc_shared::block::BlockStatesPlugin;
use rc_shared::item::{ItemStates, ItemStatesPlugin};

#[rustfmt::skip]
fn main() {
    
    App::new()
        .add_plugins(
            DefaultPlugins
            .set(LogPlugin {
                filter: "wgpu=error,rustcraft=debug,naga=error,bevy_app=info".into(),
                level: Level::INFO,
            })
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    ..default()
                })
            })
            .set(bevy::prelude::AssetPlugin {
                watch_for_changes_override: Some(true),
                file_path: "../../assets".to_string(),
                ..default()
            })
            .set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(TemporalAntiAliasPlugin)

        .insert_resource(AmbientLight {
            brightness: 5.0,
            ..default()
        })
        
        // add the app state 
        .add_state::<AppState>()
        
        .add_plugins(PolylinePlugin)

        // Networking
        .add_plugins(ClientNetworkingPlugin)
        
        // Interaction
        .add_systems(Update, mouse_interaction)
        .insert_resource(HighlightData::default())
        .add_systems(Startup, setup_highlights)
        .add_systems(Update, mouse_highlight_interaction)
        
        .add_plugins(GameEventsPlugin)
        
        // Chunk deserialisation
        .add_plugins(ChunkPlugin)

        .add_plugins(InputPlugin)

        .add_plugins(CameraPlugin)

        .add_plugins(PhysicsPlugin)

        .add_plugins(WorldPlugin)

        .add_plugins(UIPlugin)

        .add_plugins(InventoryPlugin)
        
        .insert_resource(ItemStates::new())
        
        // Asset Loaders
        .init_asset::<ResourcePacks>()
        .init_asset::<ResourcePackData>()
        .add_plugins(MaterialPlugin::<ChunkMaterial>::default())
        .init_asset_loader::<JsonAssetLoader<ResourcePacks>>()
        .init_asset_loader::<ResourcePackAssetLoader>()

        .add_plugins(BlockStatesPlugin {
            texture_atlas: &TEXTURE_ATLAS
        })
        .add_plugins(ItemStatesPlugin)
        
        .add_systems(Startup, create_states)
        .add_systems(Update, track_blockstate_changes)
        .add_systems(Update, track_itemstate_changes)
        
        // Asset deserialisation
        .add_plugins(AssetPlugin)
        .run();
}
