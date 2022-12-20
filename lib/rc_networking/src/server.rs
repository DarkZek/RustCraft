use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::time::SystemTime;
use bevy::app::AppExit;
use bevy::prelude::*;
use renet::{RenetServer, ServerAuthentication, ServerEvent};
use crate::{Channel, get_renet_connection_config, has_resource, PRIVATE_KEY, PROTOCOL_ID};
use crate::messaging::NetworkEntities;
use crate::messaging::server::{deserialize, serialize};

pub fn start_server(host_port: u16) -> Server {
    let bind_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, host_port);
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let socket = UdpSocket::bind(bind_addr).unwrap();
    let server = RenetServer::new(
        current_time,
        renet::ServerConfig {
            max_clients: 1024,
            protocol_id: PROTOCOL_ID,
            public_addr: SocketAddr::V4(bind_addr),
            authentication: ServerAuthentication::Secure {
                private_key: PRIVATE_KEY,
            },
        },
        get_renet_connection_config(),
        socket,
    );
    info!("Listening to connections on {}", bind_addr);
    Server(server.unwrap())
}

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        use bevy::prelude::CoreStage::*;
        crate::messaging::add_events(app);

        app
            .init_resource::<NetworkEntities>()
            .add_system_to_stage(PreUpdate, update_system
                .with_run_criteria(has_resource::<Server>)
            )
            .add_system_to_stage(PreUpdate, read_packets_system
                .after(update_system)
                .with_run_criteria(has_resource::<Server>)
            )
            .add_system_to_stage(PostUpdate, detect_shutdown_system
                .before(write_packets_system)
                .with_run_criteria(has_resource::<Server>)
            )
            .add_system_to_stage(PostUpdate, write_packets_system
                .with_run_criteria(has_resource::<Server>)
            )
            .add_system_to_stage(PostUpdate, send_packets_system
                .after(write_packets_system)
                .with_run_criteria(has_resource::<Server>)
            );
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

fn detect_shutdown_system(mut server: ResMut<Server>, mut bevy_shutdown: EventReader<AppExit>) {
    for _ in bevy_shutdown.iter() {
        info!("Shutting down server");
        server.disconnect_clients();
    }
}