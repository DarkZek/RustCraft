use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::Res;
use bevy_log::info;
use bevy_testing_protocol::channels::Channels;
use bevy_testing_protocol::protocol::clientbound::player_join::PlayerJoin;
use bevy_testing_protocol::protocol::Protocol;
use naia_bevy_server::events::AuthorizationEvent;
use naia_bevy_server::Server;
use crate::resources::Global;

pub fn authorization_event(
    mut event_reader: EventReader<AuthorizationEvent<Protocol>>,
    mut server: Server<Protocol, Channels>,
    mut global: ResMut<Global>
) {
    for event in event_reader.iter() {
        if let AuthorizationEvent(user_key, Protocol::UserAuthenticate(auth)) = event {
            server.accept_connection(&user_key);

            global.authentication_requests.insert(*user_key, (*auth.username).clone());
        }
    }
}