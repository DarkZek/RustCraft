use crate::state::AppState;
use bevy::prelude::*;

#[derive(Resource)]
pub struct MainMenuData {
    ui: Entity,
}

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let entity = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::rgb(0.361, 0.42, 0.753).into(),
            ..default()
        })
        .with_children(|c| {
            c.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Connect",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
        })
        .id();

    commands.insert_resource(MainMenuData { ui: entity })
}

pub fn destroy_main_menu(mut commands: Commands, menu: Res<MainMenuData>) {
    commands.entity(menu.ui).despawn_recursive();
    commands.remove_resource::<MainMenuData>();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<State<AppState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                app_state.set(AppState::Connecting).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
