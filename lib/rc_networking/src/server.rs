use bevy::prelude::*;
use renet::{RenetServer, ServerEvent};
use crate::Channel;
use crate::messaging::server::{deserialize, serialize};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        use bevy::prelude::CoreStage::*;
        crate::messaging::add_events(app);


    }
}

crate::make_wrapper_struct!(Server, RenetServer);

pub fn update_system(
    mut server: ResMut<Server>,
    mut server_events: EventWriter<ServerEvent>,
    time: Res<Time>,
) {
    if let Err(e) = server.update(time.delta()) {
        error!("Renet Update: {}", e);
    }

    while let Some(event) = server.get_event() {
        info!("{:?}", event);
        server_events.send(event);
    }
}

pub fn read_packets_system(world: &mut World) {
    world.resource_scope::<Server, _>(|world, mut server| {
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
    });
}

pub fn write_packets_system(world: &mut World) {
    world.resource_scope::<Server, _>(|world, mut server: Mut<Server>| {
        serialize(world, server.as_mut());
    })
}

pub fn send_packets_system(mut server: ResMut<Server>) {
    if let Err(e) = server.send_packets() {
        error!("Renet Send: {}", e);
    }
}
