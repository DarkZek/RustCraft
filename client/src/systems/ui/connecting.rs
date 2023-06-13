use crate::state::AppState;
use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct ConnectingData {
    pub texture_atlas: bool,
    pub block_states: bool,
    pub ui: Option<Entity>,
}

pub fn setup_connecting_ui(
    mut commands: Commands,
    mut data: ResMut<ConnectingData>,
    asset_server: Res<AssetServer>,
) {
    let ui = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(0.361, 0.42, 0.753).into(),
            ..default()
        })
        .with_children(|c| {
            c.spawn(TextBundle::from_section(
                "Loading...",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .id();

    data.ui = Some(ui);
}

pub fn remove_connecting_ui(mut commands: Commands, data: ResMut<ConnectingData>) {
    if let Some(ui) = data.ui {
        commands.entity(ui).despawn();
    }
}
