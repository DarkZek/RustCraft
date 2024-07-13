
use bevy::prelude::*;
use std::time::{Duration};
use web_time::Instant;

#[derive(Resource, Debug)]
pub struct DebuggingUIData {
    pub physics_tick_entity: Option<Entity>,
    last_update: Instant,
    pub physics_ticks: u32,
}

impl Default for DebuggingUIData {
    fn default() -> Self {
        DebuggingUIData {
            physics_tick_entity: None,
            last_update: Instant::now(),
            physics_ticks: 0,
        }
    }
}

pub fn setup_debugging_ui(
    mut commands: Commands,
    mut data: ResMut<DebuggingUIData>,
    asset_server: Res<AssetServer>,
) {
    let _ = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                position_type: PositionType::Absolute,
                align_items: AlignItems::FlexEnd,
                padding: UiRect {
                    bottom: Val::Px(50.),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|c| {
            data.physics_tick_entity = Some(
                c.spawn(TextBundle::from_section(
                    "Phy/s: 000",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ))
                .id(),
            );
        })
        .id();
}

pub fn update_debugging_ui(mut query: Query<&mut Text>, mut data: ResMut<DebuggingUIData>) {
    if data.last_update.elapsed() < Duration::from_secs(1) {
        // No update
        return;
    }

    query
        .get_mut(*data.physics_tick_entity.as_ref().unwrap())
        .unwrap()
        .sections
        .get_mut(0)
        .unwrap()
        .value = format!(
        "Phy/s: {:.00}",
        data.physics_ticks as f32 / data.last_update.elapsed().as_secs_f32()
    );

    data.last_update = Instant::now();
    data.physics_ticks = 0;
}
