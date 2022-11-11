use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui);
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(720.0), Val::Px(80.0)),
                    align_self: AlignSelf::FlexStart,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                image: asset_server.load("ui/hotbar.png").into(),
                ..default()
            });
            parent.spawn_bundle(ImageBundle {
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
