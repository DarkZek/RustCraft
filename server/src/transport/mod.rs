mod connection;

use crate::systems::authorization::GameUser;
use crate::transport::connection::accept_connections;
use bevy::app::{App, Plugin};

use rc_networking::constants::UserId;
use std::collections::{HashMap, HashSet};

use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::str::FromStr;

use crate::ServerConfig;
use bevy::ecs::prelude::Resource;
use bevy::prelude::{info, Update};
use bevy::utils::default;
use rc_networking::server::{NetworkingServerConfig, QuinnServerPlugin};
use rc_networking::*;
use std::time::SystemTime;

pub struct TransportPlugin;

#[derive(Default, Resource)]
pub struct TransportSystem {
    pub clients: HashMap<UserId, GameUser>,
    // List of clients still initialising content
    pub initialising_clients: HashSet<UserId>,
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
        let server = 0;

        info!("Listening to connections on {:?}", bind_addr);

        app.insert_resource(NetworkingServerConfig {
            address: Some(bind_addr),
            ..default()
        });

        let transport_system = TransportSystem::default();

        app.add_plugins(QuinnServerPlugin)
            .insert_resource(transport_system)
            .add_systems(Update, accept_connections);
    }
}
