pub mod connecting;
mod inventory;
pub mod loading;
pub mod main_menu;

use crate::state::AppState;
use crate::systems::ui::connecting::ConnectingData;
use crate::systems::ui::inventory::hotbar::{setup_hotbar_ui, update_hotbar_ui};
use crate::systems::ui::inventory::InventoryUI;
use crate::systems::ui::loading::{
    check_loading, remove_loading_ui, set_loading, setup_loading_ui, LoadingData,
};
use crate::systems::ui::main_menu::{button_system, destroy_main_menu, setup_main_menu};
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_ui.in_schedule(OnEnter(AppState::InGame)))
            // Main menu
            .add_system(setup_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(button_system.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(destroy_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            // Loading
            .add_startup_system(setup_loading_ui)
            .insert_resource(LoadingData::default())
            .add_system(set_loading.in_set(OnUpdate(AppState::Preloading)))
            .add_system(check_loading.in_set(OnUpdate(AppState::Loading)))
            .add_system(remove_loading_ui.in_schedule(OnExit(AppState::Loading)))
            // Inventory
            .insert_resource(InventoryUI::default())
            .add_system(setup_hotbar_ui.in_schedule(OnEnter(AppState::InGame)))
            .add_system(update_hotbar_ui)
            // Connecting
            .insert_resource(ConnectingData::default())
            .add_system(connecting::setup_connecting_ui.in_schedule(OnEnter(AppState::Connecting)))
            .add_system(connecting::remove_connecting_ui.in_schedule(OnExit(AppState::Connecting)));
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
                    size: Size::new(Val::Px(25.0), Val::Px(25.0)),
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                image: asset_server.load("ui/crosshair.png").into(),
                ..default()
            });
        });
}
