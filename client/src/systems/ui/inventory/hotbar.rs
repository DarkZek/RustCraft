use crate::game::inventory::Inventory;
use crate::systems::ui::inventory::InventoryUI;
use bevy::prelude::*;
use rc_networking::protocol::Protocol;
use rc_networking::protocol::serverbound::change_hotbar_slot::ChangeHotbarSlot;
use rc_networking::types::SendPacket;
use rc_shared::constants::UserId;

const HOTBAR_SLOTS: usize = 10;

/// Sets up the UI entities
pub fn setup_hotbar_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut inventory_ui: ResMut<InventoryUI>,
) {
    inventory_ui.hotbar_selected_images = Some([
        asset_server.load::<Image>("ui/hotbar_selected_0.png"),
        asset_server.load::<Image>("ui/hotbar_selected_1.png"),
        asset_server.load::<Image>("ui/hotbar_selected_2.png"),
        asset_server.load::<Image>("ui/hotbar_selected_3.png"),
        asset_server.load::<Image>("ui/hotbar_selected_4.png"),
        asset_server.load::<Image>("ui/hotbar_selected_5.png"),
        asset_server.load::<Image>("ui/hotbar_selected_6.png"),
        asset_server.load::<Image>("ui/hotbar_selected_7.png"),
        asset_server.load::<Image>("ui/hotbar_selected_8.png"),
        asset_server.load::<Image>("ui/hotbar_selected_9.png"),
    ]);

    let mut hotbar_selected_entity = None;
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
                    let default_image = inventory_ui.hotbar_selected_images.as_ref().unwrap().get(0).unwrap();
                    hotbar_selected_entity = Some(
                        parent
                            .spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(80.0),
                                    height: Val::Px(80.0),
                                    align_self: AlignSelf::FlexStart,
                                    position_type: PositionType::Absolute,
                                    ..default()
                                },
                                image: default_image.clone().into(),
                                ..default()
                            })
                            .id(),
                    );

                    // Spawn icons
                    for i in 0..HOTBAR_SLOTS {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Px(80.0),
                                    height: Val::Px(80.0),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {

                                hotbar_icons[i] = Some(
                                    parent
                                        .spawn(ImageBundle {
                                            style: Style {
                                                width: Val::Px(60.0),
                                                height: Val::Px(60.0),
                                                display: Display::None,
                                                top: Val::Px(10.0),
                                                left: Val::Px(10.0),
                                                right: Val::Px(10.0),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .id(),
                                );

                                hotbar_text[i] = Some(
                                    parent
                                        .spawn(TextBundle {
                                            style: Style {
                                                position_type: PositionType::Absolute,
                                                right: Val::Px(15.0),
                                                bottom: Val::Px(10.0),
                                                display: Display::None,
                                                ..default()
                                            },
                                            text: Text::from_section(
                                                "",
                                                TextStyle {
                                                    font: font_handle.clone(),
                                                    font_size: 30.0,
                                                    color: Color::WHITE,
                                                },
                                            ),
                                            ..default()
                                        })
                                        .id(),
                                );
                            });
                    }
                });
        });

    inventory_ui.hotbar_selected_entity = hotbar_selected_entity;
    inventory_ui.hotbar_icons = Some(hotbar_icons.map(|v| v.unwrap()));
    inventory_ui.hotbar_text = Some(hotbar_text.map(|v| v.unwrap()));
}

pub fn update_hotbar_ui(
    mut inventory: ResMut<Inventory>,
    inventory_ui: ResMut<InventoryUI>,
    keys: Res<ButtonInput<KeyCode>>,
    mut style: Query<&mut Style>,
    mut images: Query<&mut UiImage>,
    mut texts: Query<&mut Text>,
    asset_server: Res<AssetServer>,
    mut send_packet: EventWriter<SendPacket>
) {
    let (selected_slot_changed, hotbar_index) = get_hotbar_keypresses(&keys);

    if !selected_slot_changed && hotbar_index != inventory.hotbar_slot && !inventory.dirty {
        return;
    }

    inventory.dirty = false;

    if selected_slot_changed {
        inventory.hotbar_slot = hotbar_index;

        let mut selected_hotbar_style = style
            .get_mut(*inventory_ui.hotbar_selected_entity.as_ref().unwrap())
            .unwrap();

        selected_hotbar_style.left =
            Val::Percent((100.0 / HOTBAR_SLOTS as f32) * inventory.hotbar_slot as f32);

        let mut selected_hotbar_image = images
            .get_mut(*inventory_ui.hotbar_selected_entity.as_ref().unwrap())
            .unwrap();

        selected_hotbar_image.texture = inventory_ui.hotbar_selected_images.as_ref().unwrap().get(hotbar_index as usize).unwrap().clone().into();

        send_packet.send(SendPacket(
            Protocol::ChangeHotbarSlot(ChangeHotbarSlot::new(hotbar_index)),
            UserId(0)
        ));
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

fn get_hotbar_keypresses(keys: &Res<ButtonInput<KeyCode>>) -> (bool, u8) {
    if keys.just_pressed(KeyCode::Digit1) {
        return (true, 0);
    }
    if keys.just_pressed(KeyCode::Digit2) {
        return (true, 1);
    }
    if keys.just_pressed(KeyCode::Digit3) {
        return (true, 2);
    }
    if keys.just_pressed(KeyCode::Digit4) {
        return (true, 3);
    }
    if keys.just_pressed(KeyCode::Digit5) {
        return (true, 4);
    }
    if keys.just_pressed(KeyCode::Digit6) {
        return (true, 5);
    }
    if keys.just_pressed(KeyCode::Digit7) {
        return (true, 6);
    }
    if keys.just_pressed(KeyCode::Digit8) {
        return (true, 7);
    }
    if keys.just_pressed(KeyCode::Digit9) {
        return (true, 8);
    }
    if keys.just_pressed(KeyCode::Digit0) {
        return (true, 9);
    }

    (false, 0)
}
