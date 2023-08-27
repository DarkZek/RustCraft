use crate::game::inventory::Inventory;
use crate::game::item::states::ItemStates;
use crate::game::item::ItemStack;
use crate::systems::ui::inventory::InventoryUI;
use bevy::prelude::*;

const HOTBAR_SLOTS: usize = 10;

/// Sets up the UI entities
pub fn setup_hotbar_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut inventory_ui: ResMut<InventoryUI>,
) {
    let mut hotbar_selected_image = None;
    let mut hotbar_icons = [None, None, None, None, None, None, None, None, None, None];
    let mut hotbar_text = [None, None, None, None, None, None, None, None, None, None];

    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(800.0),
                        height: Val::Px(80.0),
                        align_self: AlignSelf::Center,
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    image: asset_server.load("ui/hotbar.png").into(),
                    ..default()
                })
                .with_children(|parent| {
                    hotbar_selected_image = Some(
                        parent
                            .spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(80.0),
                                    height: Val::Px(80.0),
                                    align_self: AlignSelf::FlexStart,
                                    position_type: PositionType::Absolute,
                                    ..default()
                                },
                                image: asset_server.load("ui/hotbar_selected.png").into(),
                                ..default()
                            })
                            .id(),
                    );

                    // Spawn icons
                    for i in 0..HOTBAR_SLOTS {
                        hotbar_icons[i] = Some(
                            parent
                                .spawn(ImageBundle {
                                    style: Style {
                                        width: Val::Px(80.0),
                                        height: Val::Px(80.0),
                                        align_self: AlignSelf::FlexStart,
                                        position_type: PositionType::Absolute,
                                        left: Val::Percent(
                                            (100.0 / (HOTBAR_SLOTS as f32)) * i as f32,
                                        ),
                                        display: Display::None,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .id(),
                        );
                    }

                    // Text
                    for i in 0..HOTBAR_SLOTS {
                        hotbar_text[i] = Some(
                            parent
                                .spawn(TextBundle {
                                    style: Style {
                                        width: Val::Px(80.0),
                                        height: Val::Px(80.0),
                                        align_self: AlignSelf::FlexStart,
                                        position_type: PositionType::Absolute,
                                        left: Val::Percent(
                                            (100.0 / (HOTBAR_SLOTS as f32)) * i as f32 + 1.0,
                                        ),
                                        right: Val::Percent(5.0),
                                        display: Display::None,
                                        ..default()
                                    },
                                    text: Text::from_section(
                                        // Accepts a String or any type that converts into a String, such as &str.
                                        "test",
                                        TextStyle {
                                            font: font_handle.clone(),
                                            font_size: 40.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                })
                                .id(),
                        );
                    }
                });
        });

    inventory_ui.hotbar_selected_image = hotbar_selected_image;
    inventory_ui.hotbar_icons = Some(hotbar_icons.map(|v| v.unwrap()));
    inventory_ui.hotbar_text = Some(hotbar_text.map(|v| v.unwrap()));
}

pub fn update_hotbar_ui(
    mut inventory: ResMut<Inventory>,
    mut inventory_ui: ResMut<InventoryUI>,
    keys: Res<Input<KeyCode>>,
    mut style: Query<&mut Style>,
    mut images: Query<&mut UiImage>,
    mut texts: Query<&mut Text>,
    asset_server: Res<AssetServer>,
) {
    let (selected_slot_changed, hotbar_index) = get_hotbar_keypresses(&keys);

    if !selected_slot_changed && hotbar_index != inventory.hotbar_slot && !inventory.dirty {
        return;
    }

    inventory.dirty = false;

    if selected_slot_changed {
        inventory.hotbar_slot = hotbar_index;

        let mut hotbar_style = style
            .get_mut(*inventory_ui.hotbar_selected_image.as_ref().unwrap())
            .unwrap();

        hotbar_style.left =
            Val::Percent((100.0 / HOTBAR_SLOTS as f32) * inventory.hotbar_slot as f32);
    }

    if inventory_ui.hotbar_icons.is_none() {
        return;
    }

    for i in 0..HOTBAR_SLOTS {
        let itemstack = inventory.hotbar[i as usize].as_ref();
        if let Some(icon) = inventory_ui.hotbar_icons.unwrap().get(i) {
            let image_asset = if itemstack.is_some() {
                style.get_mut(*icon).unwrap().display = Display::Flex;
                asset_server
                    .load(&format!("ui/icons/{}.png", itemstack.unwrap().item.icon))
                    .into()
            } else {
                style.get_mut(*icon).unwrap().display = Display::None;
                UiImage::default()
            };

            *images.get_mut(*icon).unwrap() = image_asset;
        }
        if let Some(text) = inventory_ui.hotbar_text.unwrap().get(i) {
            let string = if itemstack.is_some() {
                style.get_mut(*text).unwrap().display = Display::Flex;
                format!("{}", itemstack.unwrap().amount)
            } else {
                style.get_mut(*text).unwrap().display = Display::None;
                String::new()
            };

            texts
                .get_mut(*text)
                .unwrap()
                .sections
                .get_mut(0)
                .unwrap()
                .value = string;
        }
    }
}

fn get_hotbar_keypresses(keys: &Res<Input<KeyCode>>) -> (bool, u8) {
    if keys.just_pressed(KeyCode::Key1) {
        return (true, 0);
    }
    if keys.just_pressed(KeyCode::Key2) {
        return (true, 1);
    }
    if keys.just_pressed(KeyCode::Key3) {
        return (true, 2);
    }
    if keys.just_pressed(KeyCode::Key4) {
        return (true, 3);
    }
    if keys.just_pressed(KeyCode::Key5) {
        return (true, 4);
    }
    if keys.just_pressed(KeyCode::Key6) {
        return (true, 5);
    }
    if keys.just_pressed(KeyCode::Key7) {
        return (true, 6);
    }
    if keys.just_pressed(KeyCode::Key8) {
        return (true, 7);
    }
    if keys.just_pressed(KeyCode::Key9) {
        return (true, 8);
    }
    if keys.just_pressed(KeyCode::Key0) {
        return (true, 9);
    }

    (false, 0)
}
