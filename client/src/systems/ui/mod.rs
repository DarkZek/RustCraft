pub mod connecting;
pub mod loading;
pub mod main_menu;

use crate::state::AppState;
use crate::systems::ui::connecting::ConnectingData;
use crate::systems::ui::loading::{
    check_loading, remove_loading_ui, set_loading, setup_loading_ui, LoadingData,
};
use crate::systems::ui::main_menu::{button_system, destroy_main_menu, setup_main_menu};
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_ui))
            // Main menu
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_main_menu))
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(button_system))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(destroy_main_menu))
            // Loading
            .add_startup_system(setup_loading_ui)
            .insert_resource(LoadingData::default())
            .add_system_set(SystemSet::on_update(AppState::Preloading).with_system(set_loading))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_loading))
            .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(remove_loading_ui))
            // Connecting
            .insert_resource(ConnectingData::default())
            .add_system_set(
                SystemSet::on_enter(AppState::Connecting)
                    .with_system(connecting::setup_connecting_ui),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Connecting)
                    .with_system(connecting::remove_connecting_ui),
            );
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
