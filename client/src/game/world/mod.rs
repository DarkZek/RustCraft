use crate::game::world::destroy_block::destroy_block_system;
use crate::game::world::sun::{setup_sun, update_sun};
use crate::state::AppState;
use bevy::app::{App, Startup};
use bevy::prelude::{AssetApp, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};
use crate::game::world::static_world_data::{handle_loaded_static_world, load_main_menu_world, StaticWorldData, MainMenuWorldState, remove_static_world, save_surroundings_system};
use crate::systems::asset::parsing::message_pack::MessagePackAssetLoader;

mod destroy_block;
pub mod sun;
mod static_world_data;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_sun)
            .add_systems(
                Update,
                (update_sun, destroy_block_system),
            )
            // Main menu world functionality
            .add_systems(OnEnter(AppState::MainMenu), load_main_menu_world)
            .add_systems(OnExit(AppState::MainMenu), remove_static_world)
            .add_systems(Update, save_surroundings_system)
            .add_systems(Update, handle_loaded_static_world)
            .init_asset::<StaticWorldData>()
            .init_asset_loader::<MessagePackAssetLoader<StaticWorldData>>()
            .insert_resource(MainMenuWorldState::default());
    }
}
