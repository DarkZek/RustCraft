use crate::events::disconnect::DisconnectionEvent;
use crate::{TransportSystem, WorldData};
use bevy::ecs::event::EventReader;
use bevy::ecs::prelude::{Commands, EventWriter, Res};
use bevy::ecs::system::ResMut;
use rc_networking::protocol::clientbound::despawn_entity::DespawnEntity;

use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;

pub fn disconnection_event(
    mut event_reader: EventReader<DisconnectionEvent>,
    mut commands: Commands,
    mut world: ResMut<WorldData>,
    mut writer: EventWriter<SendPacket>,
    clients: Res<TransportSystem>,
) {
    for event in event_reader.iter() {
        if let Some(eid) = world.entities.remove(&event.user.entity_id) {
            // Delete entity
            commands.entity(eid).despawn();

            // Send all other players a disconnection event
            for (uid, _) in &clients.clients {
                writer.send(SendPacket(
                    Protocol::DespawnEntity(DespawnEntity::new(event.user.entity_id)),
                    *uid,
                ));
            }
        }
    }
}
