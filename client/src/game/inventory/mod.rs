use crate::game::inventory::hotbar::{setup_hotbar_ui, update_hotbar};
use crate::game::item::ItemStack;
use crate::state::AppState;
use bevy::app::{App, Plugin};
use bevy::prelude::*;

pub mod hotbar;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Inventory::default())
            .add_system(update_hotbar)
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_hotbar_ui));
    }
}

#[derive(Resource)]
pub struct Inventory {
    pub hotbar: [Option<ItemStack>; 10],
    pub hotbar_slot: u8,
    pub hotbar_selected_image: Option<Entity>,
}

impl Inventory {
    pub fn selected_block_id(&self) -> Option<u32> {
        if let Some(val) = &self.hotbar[self.hotbar_slot as usize] {
            val.item.block_state
        } else {
            None
        }
    }
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
