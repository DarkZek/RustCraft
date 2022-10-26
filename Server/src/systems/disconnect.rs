use bevy_ecs::event::EventReader;
use bevy_ecs::system::ResMut;
use crate::events::disconnect::DisconnectionEvent;
use crate::info;

pub fn disconnection_event(
    mut event_reader: EventReader<DisconnectionEvent>,
) {
    for event in event_reader.iter() {
        info!("Rustcraft Server disconnected from: {:?}", event.client);
    }
}