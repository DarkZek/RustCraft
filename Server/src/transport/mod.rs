mod connection;
mod listener;
pub mod packet;

use crate::error::ServerError;
use crate::events::authorization::AuthorizationEvent;
use crate::events::connection::ConnectionEvent;
use crate::events::disconnect::DisconnectionEvent;
use crate::systems::authorization::GameUser;
use crate::transport::connection::{accept_connections, check_connections, send_packets, SERVER};
use crate::transport::listener::ServerListener;
use bevy_app::{App, Plugin};
use bevy_ecs::event::EventWriter;
use bevy_ecs::system::{Res, ResMut};
use bevy_ecs::world::Mut;
use bevy_log::{info, warn};
use mio::{Events, Interest, Poll};
use rustcraft_protocol::constants::UserId;
use std::collections::HashMap;
use std::io;
use std::net::{IpAddr, TcpListener, TcpStream};
use std::process::exit;
use std::str::FromStr;

pub struct TransportPlugin;

pub struct TransportSystem {
    ip: IpAddr,
    port: usize,
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
        let ip = IpAddr::from_str("0.0.0.0").unwrap();
        let port = 25567;

        let mut stream = match ServerListener::new(ip, port) {
            Ok(val) => val,
            Err(err) => {
                panic!("{:?}", err);
            }
        };

        let transport_system = TransportSystem::new(ip, port).unwrap();

        app.insert_resource(stream)
            .insert_resource(transport_system)
            .add_system(send_packets)
            .add_system(accept_connections)
            .add_system(check_connections)
            .add_event::<ConnectionEvent>()
            .add_event::<AuthorizationEvent>()
            .add_event::<DisconnectionEvent>();
    }
}

impl TransportSystem {
    pub fn new(ip: IpAddr, port: usize) -> Result<TransportSystem, ServerError> {
        Ok(TransportSystem {
            ip,
            port,
            clients: Default::default(),
            total_connections: 0,
        })
    }
}
