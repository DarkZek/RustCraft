use crate::game::entity::{GameObject};

use crate::game::inventory::Inventory;
use crate::systems::networking::NetworkingSystem;
use crate::systems::physics::PhysicsObject;
use bevy::prelude::*;
use bevy::math::prelude::Cuboid;

use nalgebra::Vector3;
use rc_shared::game_objects::GameObjectData;
use rc_shared::item::types::ItemStack;
use rc_shared::item::ItemStates;

use crate::state::AppState;
use rc_networking::protocol::Protocol;
use rc_networking::types::ReceivePacket;
use rc_shared::aabb::Aabb;

pub fn messages_update(
    mut event_reader: EventReader<ReceivePacket>,
    mut transforms: Query<&mut Transform>,
    mut physics_objects: Query<&mut PhysicsObject>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut system: ResMut<NetworkingSystem>,
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
