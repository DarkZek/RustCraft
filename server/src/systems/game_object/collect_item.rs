use bevy::{ecs::{event::EventWriter, system::{Commands, Query}}, log::info};
use rc_networking::{protocol::{clientbound::{despawn_game_object::DespawnGameObject, update_inventory_slot::UpdateInventorySlot}, Protocol}, types::SendPacket};
use rc_shared::game_objects::GameObjectData;
use crate::game::{game_object::GameObject, player::Player, transform::Transform};
use bevy::ecs::entity::Entity;

const ITEM_COLLECTION_RADIUS: f32 = 2.0;

pub fn collect_items(
    mut query: Query<(Entity, &mut GameObject, &Transform)>,
    mut command: Commands,
    mut send_packet: EventWriter<SendPacket>) {

    let mut players = Vec::new();

    for (entity, object, transform) in query.iter() {
        if let GameObjectData::Player(t) = object.data {
            players.push((t.clone(), entity, transform.position));
        }
    }

    for (entity, mut object, transform) in query.iter_mut() {
        // Find all nearby items
        for (user_id, _, player_pos) in &mut players {
            if let GameObjectData::ItemDrop(item_name) = object.data.clone() {
                // Get distance between item and player
                let dist = (*player_pos - transform.position).magnitude();

                if dist < ITEM_COLLECTION_RADIUS {
                    // Prevent it from being collected twice
                    object.data = GameObjectData::Debug;
                    command.get_entity(entity).unwrap().despawn();

                    // Add to user inventory
                    let update = UpdateInventorySlot::new(item_name.clone(), 1, 1);
                    send_packet.send(SendPacket(Protocol::UpdateInventorySlot(update), *user_id));

                    send_packet.send(SendPacket(Protocol::DespawnGameObject(DespawnGameObject::new(object.id)), *user_id));
                }
            }
        }
    }
}