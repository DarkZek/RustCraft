use bevy::ecs::{component::Component, event::EventWriter, system::{Query, Resource}};
use rc_networking::{protocol::{clientbound::update_inventory::UpdateInventory, Protocol}, types::SendPacket};
use rc_shared::{constants::UserId, game_objects::GameObjectData, item::types::ItemStack};

use super::game_object::{self, GameObject};


#[derive(Component)]
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
    pub fn take_selected_block(&mut self) -> Option<u32> {
        if let Some(val) = &mut self.hotbar[self.hotbar_slot as usize] {
            let block_state = val.item.block_state;

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

pub fn propagate_inventories(
    mut query: Query<(&mut Inventory, &GameObject)>,
    mut send_packet: EventWriter<SendPacket>) {
    for (mut inventory, game_object) in query.iter_mut() {
        if !inventory.dirty {
            continue;
        }

        if let GameObjectData::Player(user_id) = game_object.data {
            inventory.dirty = false;

            // Let client know new inventory status
            let packet = UpdateInventory::new(inventory.hotbar.clone(), None);
            send_packet.send(SendPacket(Protocol::UpdateInventory(packet), user_id));
        }
    }
}