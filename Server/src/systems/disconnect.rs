use bevy_ecs::event::EventReader;
use bevy_ecs::system::ResMut;
use bevy_testing_protocol::channels::Channels;
use bevy_testing_protocol::protocol::Protocol;
use naia_bevy_server::events::DisconnectionEvent;
use naia_bevy_server::Server;
use crate::info;
use crate::resources::Global;

pub fn disconnection_event(
    mut event_reader: EventReader<DisconnectionEvent>,
    mut global: ResMut<Global>,
    mut server: Server<Protocol, Channels>,
) {
    for event in event_reader.iter() {
        let DisconnectionEvent(user_key, user) = event;
        info!("Naia Server disconnected from: {:?}", user.address);

        if let Some(entity) = global.user_to_prediction_map.remove(user_key) {
            server
                .entity_mut(&entity.entity)
                .leave_room(&global.main_room_key)
                .despawn();
        }
    }
}