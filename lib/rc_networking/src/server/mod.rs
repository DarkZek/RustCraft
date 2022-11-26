use std::io::Cursor;
use std::marker::PhantomData;
use bevy::prelude::*;
use renet::{RenetError, RenetServer, ServerEvent};
use serde::{Deserialize, Serialize};
use crate::{Channel, impl_message};
use crate::messaging::server::*;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

crate::make_wrapper_struct!(Server, RenetServer);

pub fn update_system(
    mut server: ResMut<Server>,
    mut renet_error: EventWriter<RenetError>,
    mut server_events: EventWriter<ServerEvent>,
    time: Res<Time>,
) {
    if let Err(e) = server.update(time.delta()) {
        error!("Renet Update: {}", e);
        renet_error.send(RenetError::IO(e));
    }

    while let Some(event) = server.get_event() {
        info!("{:?}", event);
        server_events.send(event);
    }
}

pub fn read_packets_system(world: &mut World, mut server: ResMut<Server>) {
    for channel in Channel::ALL {
        server
            .clients_id()
            .iter()
            .for_each(|&client_id| {
                while let Some(bytes) = server.receive_message(client_id, channel) {
                    deserialize(world, bytes, client_id);
                }
            })
    }
}

pub fn send_packets_system(mut server: ResMut<Server>) {
    
}