use bevy::{ecs::{event::EventWriter, system::{Commands, Query}}};
use rc_networking::{protocol::{clientbound::{despawn_game_object::DespawnGameObject}, Protocol}, types::SendPacket};
use rc_shared::game_objects::GameObjectData;
use crate::game::{game_object::GameObject, inventory::Inventory, transform::Transform};
use bevy::ecs::entity::Entity;

const ITEM_COLLECTION_RADIUS: f32 = 2.0;

pub fn collect_items(
    mut query: Query<(Entity, &mut GameObject, &Transform)>,
    mut inventory_query: Query<&mut Inventory>,
    mut command: Commands,
    mut send_packet: EventWriter<SendPacket>) {

    let mut players = Vec::new();

    for (entity, object, transform) in query.iter() {
        if let GameObjectData::Player(t) = object.data {
            players.push((t.clone(), entity, transform.position));
        }
    }

    for (entity, mut object, transform) in query.iter_mut() {
        if let GameObjectData::ItemDrop(item) = object.data.clone() {
            // Find all nearby items
            for (user_id, player_entity, player_pos) in &mut players {
                // Get distance between item and player
                let dist = (*player_pos - transform.position).magnitude();

                if dist < ITEM_COLLECTION_RADIUS {
                    // Prevent it from being collected twice
                    object.data = GameObjectData::Debug;
                    command.get_entity(entity).unwrap().despawn();

                    // Add to user inventory
                    let mut inventory = inventory_query.get_mut(*player_entity).unwrap();
                    inventory.push_item(item.clone());

                    send_packet.send(SendPacket(Protocol::DespawnGameObject(DespawnGameObject::new(object.id)), *user_id));
                }
            }
        }
    }
}