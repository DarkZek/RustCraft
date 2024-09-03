#![allow(dead_code)]

pub mod game;
pub mod state;
pub mod systems;
pub mod utils;

use crate::game::events::GameEventsPlugin;
use crate::game::game_object::GameObjectPlugin;
use crate::game::interaction::highlight::{
    mouse_highlight_interaction, setup_highlights, HighlightData,
};
use crate::game::inventory::InventoryPlugin;
use crate::game::state::{create_states, track_blockstate_changes, track_itemstate_changes};
use crate::game::world::WorldPlugin;
use crate::state::AppState;
use crate::systems::asset::atlas::atlas::TEXTURE_ATLAS;
use crate::systems::asset::atlas::resource_packs::{ResourcePackData, ResourcePacks};
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
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use rc_shared::block::BlockStatesPlugin;
use rc_shared::item::{ItemStates, ItemStatesPlugin};
use bevy::pbr::ExtendedMaterial;
use crate::game::interaction::InteractionPlugin;
use crate::systems::asset::material::chunk_extension::ChunkMaterialExtension;
use crate::systems::asset::material::translucent_chunk_extension::TranslucentChunkMaterialExtension;

// TODO: Performance - Make event based systems only run on event trigger https://docs.rs/bevy/latest/bevy/ecs/prelude/fn.on_event.html

#[rustfmt::skip]
fn main() {

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "wgpu=error,naga=error,bevy_app=info".into(),
                    level: Level::DEBUG,
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        ..default()
                    }),
                    synchronous_pipeline_compilation: false,
                })
                .set(bevy::prelude::AssetPlugin {
                    watch_for_changes_override: Some(true),
                    file_path: "../assets".to_string(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()))

        .add_plugins(TemporalAntiAliasPlugin)

        .insert_resource(AmbientLight {
            brightness: 175.0,
            color: Color::srgb(0.95, 0.95, 1.0)
        })

        // add the app state
        .init_state::<AppState>()

        // Networking
        .add_plugins(ClientNetworkingPlugin)

        // Interaction
        .add_plugins(InteractionPlugin)
        .insert_resource(HighlightData::default())
        .add_systems(Startup, setup_highlights)
        .add_systems(Update, mouse_highlight_interaction)

        .add_plugins(GameEventsPlugin)
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
        .add_plugins(MaterialPlugin::<ExtendedMaterial<StandardMaterial, ChunkMaterialExtension>>::default())
        .add_plugins(MaterialPlugin::<ExtendedMaterial<StandardMaterial, TranslucentChunkMaterialExtension>>::default())
        .init_asset_loader::<JsonAssetLoader<ResourcePacks>>()
        .init_asset_loader::<ResourcePackAssetLoader>()

        .add_plugins(BlockStatesPlugin {
            texture_atlas: &TEXTURE_ATLAS
        })
        .add_plugins(ItemStatesPlugin)
        .add_plugins(GameObjectPlugin)

        .add_systems(Startup, create_states)
        .add_systems(Update, track_blockstate_changes)
        .add_systems(Update, track_itemstate_changes)

        // Asset deserialisation
        .add_plugins(AssetPlugin)
        .run();
}