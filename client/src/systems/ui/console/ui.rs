use bevy::color::Color;
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::{BackgroundColor, BuildChildren, Commands, default, Entity, EventReader, FlexDirection, JustifyContent, KeyCode, NodeBundle, Query, ResMut, Style, Text, TextBundle, TextStyle, UiRect, Val, Visibility};
use crate::systems::ui::console::{ConsoleData, MAX_CHAT_LENGTH, MAX_CONSOLE_HISTORY};

pub fn setup_console_ui(
    mut commands: Commands
) {

    let style = TextStyle::default();

    let mut entity_commands = commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                padding: UiRect::new(
                    Val::Px(20.0),
                    Val::Px(20.0),
                    Val::Px(20.0),
                    Val::Px(80.0)
                ),
                ..default()
            },
            ..default()
        }
    );

    let mut ui = None;
    let mut text_prompt = None;
    let mut text_history = None;
    let mut text_history_children_texts = None;
    let mut text_history_children_items = None;

    // Spawn history ui
    entity_commands.with_children(|child_commands| {
        let mut text_history_commands = child_commands.spawn(
            NodeBundle {
                style: Style {
                    width: Val::Px(600.0),
                    max_height: Val::Vh(80.0),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                ..default()
            }
        );

        text_history_commands.with_children(|child_commands| {

            let mut children_items = Vec::new();
            let mut children_texts = Vec::new();

            for _ in 0..MAX_CONSOLE_HISTORY {
                let mut text_entry_commands = child_commands.spawn(
                    TextBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(8.0)),
                            ..default()
                        },
                        visibility: Visibility::Hidden,
                        ..default()
                    }
                );

                text_entry_commands.with_children(|child_commands| {
                    let text_entry_text_commands = child_commands.spawn(
                        TextBundle {
                            text: Text::from_section("", TextStyle::default()),
                            ..default()
                        }
                    );

                    children_texts.push(text_entry_text_commands.id());
                });

                children_items.push(text_entry_commands.id());
            }

            text_history_children_texts = Some(children_texts);
            text_history_children_items = Some(children_items);
        });

        text_history = Some(text_history_commands.id());
    });

    // Spawn textbox ui
    entity_commands.with_children(|child_commands| {
        let mut ui_commands = child_commands.spawn(
            NodeBundle {
                style: Style {
                    width: Val::Px(600.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    padding: UiRect::new(
                        Val::Px(10.0),
                        Val::Px(10.0),
                        Val::Px(20.0),
                        Val::Px(20.0),
                    ),
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.6).into(),
                visibility: Visibility::Hidden,
                ..default()
            }
        );

        ui_commands.with_children(|child_commands| {

            let text_prompt_commands = child_commands.spawn(
                TextBundle {
                    text: Text::from_section("> ", style),
                    ..default()
                }
            );

            text_prompt = Some(text_prompt_commands.id());
        });

        ui = Some(ui_commands.id());
    });

    let resource = ConsoleData {
        ui: ui.unwrap(),
        text_prompt: text_prompt.unwrap(),
        capturing: false,
        prompt_text: "".to_string(),
        history: Default::default(),
        dirty: false,
        text_history: text_history.unwrap(),
        text_history_children_texts: text_history_children_texts.unwrap(),
        text_history_children_items: text_history_children_items.unwrap(),
        commands_executed: vec![],
    };

    commands.insert_resource(resource)
}

pub fn handle_keyboard_input(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut data: ResMut<ConsoleData>,
    mut visibility: Query<&mut Visibility>
) {
    for ev in evr_kbd.read() {
        // We don't care about key releases, only key presses
        if ev.state == ButtonState::Released {
            continue;
        }

        if ev.key_code == KeyCode::Enter && !data.capturing {
            data.capture(&mut visibility);
            return
        }
        if ev.key_code == KeyCode::Escape {
            data.uncapture(&mut visibility);
            return
        }

        if !data.capturing {
            return
        }

        if let Key::Character(char) = &ev.logical_key {
            if data.prompt_text.len() > MAX_CHAT_LENGTH {
                continue
            }
            data.prompt_text += char;
            data.dirty = true;
        }

        if ev.key_code == KeyCode::Backspace {
            data.prompt_text.pop();
            data.dirty = true;
        }

        if ev.key_code == KeyCode::Space {
            data.prompt_text += " ";
            data.dirty = true;
        }

        if ev.key_code == KeyCode::Enter {

            // Treat as exiting
            if data.prompt_text.is_empty() {
                data.uncapture(&mut visibility);
                continue;
            }

            let text = data.prompt_text.clone();
            data.prompt_text.clear();
            data.dirty = true;

            data.execute_command(&text);
            data.uncapture(&mut visibility);
        }
    }
}

pub fn update_ui(
    mut data: ResMut<ConsoleData>,
    mut query: Query<(&mut Text, &mut BackgroundColor, &mut Visibility)>,
) {
    if !data.dirty {
        return
    }

    query.get_mut(data.text_prompt).unwrap().0.sections.get_mut(0).unwrap().value =
        format!("> {}", data.prompt_text.clone());

    for i in 0..MAX_CONSOLE_HISTORY {
        let item_entity = data.text_history_children_items.get(i).unwrap().clone();
        let text_entity = data.text_history_children_texts.get(i).unwrap().clone();

        if let Some(val) = data.history.get(i) {
            if !val.expired || data.capturing {
                update_history_item(
                    &mut query,
                    item_entity,
                    text_entity,
                    Some(Visibility::Visible),
                    Some(&val.message),
                    Some(val.text_color),
                    Some(val.background_color)
                );
            } else {
                update_history_item(
                    &mut query,
                    item_entity,
                    text_entity,
                    Some(Visibility::Hidden),
                    None,
                    None,
                    None
                );
            }
        } else {
            update_history_item(
                &mut query,
                item_entity,
                text_entity,
                Some(Visibility::Hidden),
                None,
                None,
                None
            );
        }
    }

    data.dirty = false;
}

fn update_history_item(
    query: &mut Query<(&mut Text, &mut BackgroundColor, &mut Visibility)>,
    item_entity: Entity,
    text_entity: Entity,
    visibility: Option<Visibility>,
    message: Option<&str>,
    text_color: Option<Color>,
    background_color: Option<Color>,
) {

    if let Some(message) = message {
        query.get_mut(text_entity.clone()).unwrap().0.sections.get_mut(0).unwrap().value =
            message.to_string();
    }
    if let Some(text_color) = text_color {
        query.get_mut(text_entity).unwrap().0.sections.get_mut(0).unwrap().style.color =
            text_color;
    }

    if let Some(background_color) = background_color {
        *query.get_mut(item_entity.clone()).unwrap().1 = background_color.into();
    }

    if let Some(visibility) = visibility {
        *query.get_mut(item_entity.clone()).unwrap().2 = visibility;
    }
}