use std::time::SystemTime;
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::Res;
use bevy_log::info;
use rustcraft_protocol::protocol::clientbound::ping::Ping;
use crate::events::authorization::AuthorizationEvent;
use crate::resources::World;
use mio::net::TcpStream;
use rustcraft_protocol::protocol::serverbound::pong::Pong;
use rustcraft_protocol::stream::GameStream;

/// A user who is yet to be authorized
pub struct UnauthorizedUser {
    pub name: Option<String>,
    pub stream: GameStream,

    pub last_ping: Ping,
    pub last_pong: Pong,

    /* If the user has been disconnected */
    pub disconnected: bool
    /* Todo: Encryption */
}

impl UnauthorizedUser {
    pub fn new(stream: TcpStream) -> UnauthorizedUser {
        UnauthorizedUser {
            name: None,
            stream: GameStream::new(stream),
            last_ping: Ping::new(),
            last_pong: Pong::new(),
            disconnected: false
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
}

pub fn authorization_event(
    mut event_reader: EventReader<AuthorizationEvent>,
    mut global: ResMut<World>
) {
    // Send world to client
}