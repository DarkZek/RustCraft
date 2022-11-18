use bevy::prelude::*;

pub fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.2).into(),
            ..default()
        })
        .with_children(|c| {
            c.spawn(ButtonBundle { ..default() });
        });
}
