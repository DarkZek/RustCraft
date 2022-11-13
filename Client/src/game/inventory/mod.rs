use crate::game::inventory::hotbar::{setup_hotbar_ui, update_hotbar};
use crate::game::item::ItemStack;
use bevy::app::{App, Plugin};
use bevy::prelude::Entity;

pub mod hotbar;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Inventory::default())
            .add_system(update_hotbar)
            .add_startup_system(setup_hotbar_ui);
    }
}

pub struct Inventory {
    pub hotbar: [Option<ItemStack>; 10],
    pub hotbar_slot: u8,
    pub hotbar_selected_image: Option<Entity>,
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            hotbar: [None, None, None, None, None, None, None, None, None, None],
            hotbar_slot: 0,
            hotbar_selected_image: None,
        }
    }
}
