use crate::state::AppState;
use bevy::prelude::*;
use std::time::{Duration, Instant};

#[derive(Resource, Debug)]
pub struct FpsData {
    pub entity: Option<Entity>,
    last_update: Instant,
    frames: u32,
}

impl Default for FpsData {
    fn default() -> Self {
        FpsData {
            entity: None,
            last_update: Instant::now(),
            frames: 0,
        }
    }
}

pub fn setup_fps_ui(
    mut commands: Commands,
    mut data: ResMut<FpsData>,
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
                ..default()
            },
            ..default()
        })
        .with_children(|c| {
            data.entity = Some(
                c.spawn(TextBundle::from_section(
                    "FPS: 000",
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

pub fn update_fps_ui(mut query: Query<&mut Text>, mut data: ResMut<FpsData>, time: Res<Time>) {
    data.frames += 1;

    if data.last_update.elapsed() < Duration::from_secs(1) {
        // No update
        return;
    }

    query
        .get_mut(*data.entity.as_ref().unwrap())
        .unwrap()
        .sections
        .get_mut(0)
        .unwrap()
        .value = format!(
        "FPS: {:.00}",
        data.frames as f32 / data.last_update.elapsed().as_secs_f32()
    );

    data.last_update = Instant::now();
    data.frames = 0;
}
