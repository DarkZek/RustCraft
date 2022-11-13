use crate::game::inventory::Inventory;
use crate::game::item::states::ItemStates;
use crate::game::item::ItemStack;
use bevy::prelude::*;

pub fn setup_hotbar_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut inventory: ResMut<Inventory>,
    itemstates: Res<ItemStates>,
) {
    let mut hotbar_selected_image = None;
    let mut hotbar_icons = [None, None, None, None, None, None, None, None, None, None];

    inventory.hotbar[0] = Some(ItemStack::new(
        (*itemstates.states.get(0).unwrap()).clone(),
        1,
    ));
    inventory.hotbar[1] = Some(ItemStack::new(
        (*itemstates.states.get(1).unwrap()).clone(),
        1,
    ));

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Px(800.0), Val::Px(80.0)),
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
                            .spawn_bundle(ImageBundle {
                                style: Style {
                                    size: Size::new(Val::Px(80.0), Val::Px(80.0)),
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
                    for i in 0..10 {
                        hotbar_icons[i] = Some(
                            parent
                                .spawn_bundle(ImageBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(80.0), Val::Px(80.0)),
                                        align_self: AlignSelf::FlexStart,
                                        position_type: PositionType::Absolute,
                                        position: UiRect::new(
                                            Val::Percent((100.0 / 10.0) * i as f32),
                                            Val::Auto,
                                            Val::Auto,
                                            Val::Auto,
                                        ),
                                        display: if inventory.hotbar[i as usize].is_some() {
                                            Display::Flex
                                        } else {
                                            Display::None
                                        },
                                        ..default()
                                    },
                                    image: if inventory.hotbar[i as usize].is_some() {
                                        asset_server
                                            .load(&format!(
                                                "ui/icons/{}.png",
                                                inventory.hotbar[i as usize]
                                                    .as_ref()
                                                    .unwrap()
                                                    .item
                                                    .icon
                                            ))
                                            .into()
                                    } else {
                                        UiImage::default()
                                    },
                                    ..default()
                                })
                                .id(),
                        );
                    }
                });
        });

    inventory.hotbar_selected_image = hotbar_selected_image;
}

pub fn update_hotbar(
    mut inventory: ResMut<Inventory>,
    keys: Res<Input<KeyCode>>,
    mut style: Query<&mut Style>,
) {
    let (changed, hotbar_index) = get_hotbar_keypresses(&keys);

    if !changed
        || hotbar_index == inventory.hotbar_slot
        || inventory.hotbar_selected_image.is_none()
    {
        return;
    }

    inventory.hotbar_slot = hotbar_index;

    let mut style = style
        .get_mut(*inventory.hotbar_selected_image.as_ref().unwrap())
        .unwrap();

    style.position = UiRect::new(
        Val::Percent((100.0 / 10.0) * inventory.hotbar_slot as f32),
        Val::Auto,
        Val::Auto,
        Val::Auto,
    );
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
