use std::sync::OnceLock;
use bevy_mod_billboard::prelude::BillboardPlugin;
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
use crate::systems::networking::NetworkingPlugin;
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
use wasm_bindgen::JsValue;
use crate::authentication::GameAuthentication;
use crate::game::disconnect::on_disconnect;
use crate::game::interaction::InteractionPlugin;
use crate::systems::api::ApiPlugin;
use crate::systems::asset::material::chunk_extension::ChunkMaterialExtension;
use crate::systems::asset::material::translucent_chunk_extension::TranslucentChunkMaterialExtension;
use crate::systems::connection::ConnectionPlugin;
use web_sys::Worker;
use rc_shared::PHYSICS_SYNC_RATE_SECONDS;
use crate::game::game_mode::GameModePlugin;
use crate::systems::debugging::DebuggingPlugin;
use crate::systems::wasm::WasmPlugin;

#[rustfmt::skip]
pub fn start() {

    let authentication = GameAuthentication::get();

    App::new()
        .insert_resource(authentication)
        .insert_resource(Time::<Fixed>::from_seconds(PHYSICS_SYNC_RATE_SECONDS))
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "wgpu=error,naga=error,bevy_app=info".into(),
                    level: Level::INFO,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent:true,
                        #[cfg(target_arch = "wasm32")]
                        canvas: Some("#game".into()),
                        prevent_default_event_handling: false,
                        ..default()
                    }),
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

        .add_plugins(BillboardPlugin)
        .add_plugins(TemporalAntiAliasPlugin)

        .insert_resource(AmbientLight {
            brightness: 450.0,
            color: Color::srgb(0.95, 0.95, 1.0)
        })

        // add the app state
        .init_state::<AppState>()

        // Networking
        .add_plugins(NetworkingPlugin)

        // Interaction
        .add_plugins(InteractionPlugin)
        .insert_resource(HighlightData::default())
        .add_systems(Startup, setup_highlights)
        .add_systems(Update, mouse_highlight_interaction)
        .add_systems(OnEnter(AppState::MainMenu), on_disconnect)

        .add_plugins(ConnectionPlugin)
        .add_plugins(ApiPlugin)
        .add_plugins(GameEventsPlugin)
        .add_plugins(ChunkPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(InventoryPlugin)
        .add_plugins(GameModePlugin)
        .add_plugins(DebuggingPlugin)

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
        .add_plugins(WasmPlugin)

        .add_systems(Startup, create_states)
        .add_systems(Update, track_blockstate_changes)
        .add_systems(Update, track_itemstate_changes)

        // Asset deserialisation
        .add_plugins(AssetPlugin)
        .run();
}

unsafe impl Send for WasmContext {}
unsafe impl Sync for WasmContext {}

pub static WASM_CONTEXT: OnceLock<WasmContext> = OnceLock::new();

pub struct WasmContext {
    pub chunk_worker: Worker,
    pub startup_callback: js_sys::Function
}