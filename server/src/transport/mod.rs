mod connection;

use crate::events::authorization::AuthorizationEvent;
use crate::events::connection::ConnectionEvent;
use crate::events::disconnect::DisconnectionEvent;
use crate::systems::authorization::GameUser;
use crate::transport::connection::accept_connections;
use bevy::app::{App, Plugin};

use rc_networking::constants::UserId;
use std::collections::HashMap;

use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::str::FromStr;

use crate::ServerConfig;
use bevy::ecs::prelude::Resource;
use bevy::prelude::info;
use rc_networking::renet::{RenetServer, ServerAuthentication};
use rc_networking::*;
use std::time::SystemTime;

pub struct TransportPlugin;

#[derive(Default, Resource)]
pub struct TransportSystem {
    pub clients: HashMap<UserId, GameUser>,
    total_connections: usize,
}

impl Default for TransportPlugin {
    fn default() -> Self {
        TransportPlugin
    }
}

impl Plugin for TransportPlugin {
    fn build(&self, app: &mut App) {
        let bind_addr = {
            let settings = app.world.get_resource::<ServerConfig>().unwrap();

            SocketAddr::new(IpAddr::from_str(&settings.ip).unwrap(), settings.port)
        };

        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let socket = UdpSocket::bind(bind_addr).unwrap();
        let server = RenetServer::new(
            current_time,
            renet::ServerConfig {
                max_clients: 1024,
                protocol_id: PROTOCOL_ID,
                public_addr: bind_addr,
                authentication: ServerAuthentication::Secure {
                    private_key: PRIVATE_KEY,
                },
            },
            get_renet_connection_config(),
            socket,
        )
        .unwrap();

        info!("Listening to connections on {:?}", bind_addr);

        let transport_system = TransportSystem::default();

        app.add_plugin(RenetServerPlugin)
            .insert_resource(Server(server))
            .insert_resource(transport_system)
            .add_system(accept_connections)
            .add_event::<ConnectionEvent>()
            .add_event::<AuthorizationEvent>()
            .add_event::<DisconnectionEvent>();
    }
}
