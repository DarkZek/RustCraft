use crate::events::disconnect::DisconnectionEvent;
use crate::{info, SendPacket, TransportSystem, World};
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::{Commands, EventWriter, Res};
use bevy_ecs::system::ResMut;
use rustcraft_protocol::protocol::clientbound::despawn_entity::DespawnEntity;
use rustcraft_protocol::protocol::serverbound::disconnect::Disconnect;
use rustcraft_protocol::protocol::Protocol;

pub fn disconnection_event(
    mut event_reader: EventReader<DisconnectionEvent>,
    mut commands: Commands,
    mut world: ResMut<World>,
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
