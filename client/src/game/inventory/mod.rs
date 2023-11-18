use crate::game::item::ItemStack;

use bevy::app::{App, Plugin};
use bevy::prelude::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Inventory::default());
    }
}

#[derive(Resource)]
pub struct Inventory {
    pub hotbar: [Option<ItemStack>; 10],
    pub hotbar_slot: u8,
    pub dirty: bool,
}

impl Inventory {
    /// Gets the select block's, block id
    pub fn selected_block_id(&self) -> Option<u32> {
        if let Some(val) = &self.hotbar[self.hotbar_slot as usize] {
            val.item.block_state
        } else {
            None
        }
    }

    /// Takes one of the selected block and removes it from the inventory
    pub fn take_selected_block_id(&mut self) -> Option<u32> {
        if let Some(val) = &mut self.hotbar[self.hotbar_slot as usize] {
            let block_state = val.item.block_state;

            // Reduce amount
            val.amount -= 1;

            // Delete item if none left
            if val.amount == 0 {
                self.hotbar[self.hotbar_slot as usize] = None;
            }

            self.dirty = true;

            block_state
        } else {
            None
        }
    }

    /// Pushes an item into the inventory. Returns true if inserted, false if no space
    pub fn push_item(&mut self, item: ItemStack) -> bool {
        // Find existing itemstack and try add item to it
        for i in 0..10 {
            let common_itemstack = self.hotbar[i]
                .as_ref()
                .map(|i| i.item.name == item.item.name)
                .unwrap_or(false);
            if common_itemstack {
                // Matching item, increase amount
                self.hotbar[i].as_mut().unwrap().amount += item.amount;
                self.dirty = true;
                return true;
            }
        }

        // Create existing itemstack
        for i in 0..10 {
            if self.hotbar[i].is_none() {
                self.hotbar[i] = Some(item);
                return true;
            }
        }

        // No space
        return false;
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            hotbar: [None, None, None, None, None, None, None, None, None, None],
            hotbar_slot: 0,
            dirty: false,
        }
    }
}
