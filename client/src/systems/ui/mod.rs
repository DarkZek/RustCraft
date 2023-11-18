pub mod connecting;
mod fps;
mod inventory;
pub mod loading;
pub mod main_menu;

pub mod debugging;

use crate::state::AppState;
use crate::systems::ui::connecting::ConnectingData;
use crate::systems::ui::debugging::{setup_debugging_ui, update_debugging_ui, DebuggingUIData};
use crate::systems::ui::fps::{setup_fps_ui, update_fps_ui, FpsUIData};
use crate::systems::ui::inventory::hotbar::{setup_hotbar_ui, update_hotbar_ui};
use crate::systems::ui::inventory::InventoryUI;
use crate::systems::ui::loading::{
    check_loading, remove_loading_ui, set_loading, setup_loading_ui, LoadingUIData,
};
use crate::systems::ui::main_menu::{button_system, destroy_main_menu, setup_main_menu};
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_ui)
            // Main menu
            .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(Update, button_system.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), destroy_main_menu)
            // Loading
            .insert_resource(LoadingUIData::default())
            .add_systems(Startup, setup_loading_ui)
            .add_systems(Update, set_loading.run_if(in_state(AppState::Preloading)))
            .add_systems(Update, check_loading.run_if(in_state(AppState::Loading)))
            .add_systems(OnExit(AppState::Loading), remove_loading_ui)
            // Fps
            .insert_resource(FpsUIData::default())
            .add_systems(Startup, setup_fps_ui)
            .add_systems(Update, update_fps_ui)
            // Inventory
            .insert_resource(InventoryUI::default())
            .add_systems(OnEnter(AppState::InGame), setup_hotbar_ui)
            .add_systems(Update, update_hotbar_ui)
            // Debugging
            .insert_resource(DebuggingUIData::default())
            .add_systems(Startup, setup_debugging_ui)
            .add_systems(Update, update_debugging_ui)
            // Connecting
            .insert_resource(ConnectingData::default())
            .add_systems(
                OnEnter(AppState::Connecting),
                connecting::setup_connecting_ui,
            )
            .add_systems(
                OnExit(AppState::Connecting),
                connecting::remove_connecting_ui,
            );
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(25.0),
                    height: Val::Px(25.0),
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                image: asset_server.load("ui/crosshair.png").into(),
                ..default()
            });
        });
}
