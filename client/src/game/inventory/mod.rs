use bevy::prelude::*;
use rc_shared::block::BlockDefinitionIndex;
use rc_shared::block::definition::BlockDefinition;
use rc_shared::item::types::ItemStack;

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
    /// Gets the block definition
    pub fn selected_block_definition_index(&self) -> Option<BlockDefinitionIndex> {
        self.selected_item()
            .map(|f| f.item.block_definition_index)
            .flatten()
    }

    /// Gets the select block's, block id
    pub fn selected_item(&self) -> Option<&ItemStack> {
        self.hotbar[self.hotbar_slot as usize].as_ref().map(|data| data)
    }

    /// Takes one of the selected block and removes it from the inventory
    pub fn take_selected_block(&mut self) -> Option<BlockDefinitionIndex> {
        if let Some(val) = &mut self.hotbar[self.hotbar_slot as usize] {
            let block_state = val.item.block_definition_index;

            // Reduce amount
            val.amount -= 1;

            // Delete type if none left
            if val.amount == 0 {
                self.hotbar[self.hotbar_slot as usize] = None;
            }

            self.dirty = true;

            block_state
        } else {
            None
        }
    }

    pub fn put_slot(&mut self, content: Option<ItemStack>, slot: usize) {
        self.hotbar[slot] = content;
        self.dirty = true;
    }

    /// Pushes an type into the inventory. Returns true if inserted, false if no space
    pub fn push_item(&mut self, item: ItemStack) -> bool {
        // Find existing itemstack and try add type to it
        for i in 0..10 {
            let common_itemstack = self.hotbar[i]
                .as_ref()
                .map(|i| i.item.name == item.item.name)
                .unwrap_or(false);
            
            if common_itemstack {
                // Matching type, increase amount
                self.hotbar[i].as_mut().unwrap().amount += item.amount;
                self.dirty = true;
                return true;
            }
        }

        // Create existing itemstack
        for i in 0..10 {
            if self.hotbar[i].is_none() {
                self.hotbar[i] = Some(item);
                self.dirty = true;
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
