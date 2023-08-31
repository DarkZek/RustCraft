use crate::{TransportSystem, WorldData};
use bevy::ecs::event::EventReader;
use bevy::ecs::prelude::{Commands, EventWriter, Res};
use bevy::ecs::system::ResMut;
use rc_networking::events::disconnect::NetworkDisconnectionEvent;
use rc_networking::protocol::clientbound::despawn_entity::DespawnEntity;

use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;

pub fn disconnection_event(
    mut event_reader: EventReader<NetworkDisconnectionEvent>,
    mut commands: Commands,
    mut world: ResMut<WorldData>,
    mut writer: EventWriter<SendPacket>,
    mut clients: ResMut<TransportSystem>,
) {
    for event in event_reader.iter() {
        let entry = clients.clients.remove(&event.client).unwrap();

        if let Some(eid) = world.entities.remove(&entry.entity_id) {
            // Delete entity
            commands.entity(eid).despawn();

            // Send all other players a disconnection event
            for (uid, _) in &clients.clients {
                writer.send(SendPacket(
                    Protocol::DespawnEntity(DespawnEntity::new(entry.entity_id)),
                    *uid,
                ));
            }
        }
    }
}
