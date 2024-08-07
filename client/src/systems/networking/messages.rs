
use crate::game::inventory::Inventory;
use bevy::prelude::*;

use rc_shared::item::types::ItemStack;
use rc_shared::item::ItemStates;

use crate::state::AppState;
use rc_networking::protocol::Protocol;
use rc_networking::types::ReceivePacket;

pub fn messages_update(
    mut event_reader: EventReader<ReceivePacket>,
    mut app_state: ResMut<NextState<AppState>>,
    mut inventory: ResMut<Inventory>,
    item_state: Res<ItemStates>
) {
    for event in event_reader.read() {
        match &event.0 {
            Protocol::UpdateLoading(update) => {
                if update.loading {
                    app_state.set(AppState::Connecting);
                } else {
                    app_state.set(AppState::InGame);
                }
            }
            Protocol::UpdateInventorySlot(slot) => {
                if slot.id == "" {
                    inventory.put_slot(None, slot.slot as usize);
                } else {
                    let item_type = item_state.get_by_id(&slot.id).unwrap();
                    let item = ItemStack::new(item_type.1.clone(), slot.amount);
                    inventory.put_slot(Some(item), slot.slot as usize);
                }
            }
            Protocol::UpdateInventory(message) => {
                inventory.hotbar = message.slots.clone();
                if let Some(selected_slot) = message.hotbar_slot {
                    inventory.hotbar_slot = selected_slot;
                }
                inventory.dirty = true;
            }
            _ => {}
        }
    }
}
