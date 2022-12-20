use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, ToSocketAddrs, UdpSocket};
use std::time::SystemTime;
use bevy::app::AppExit;
use bevy::prelude::*;
use renet::{ClientAuthentication, RenetClient};
use crate::{Channel, get_renet_connection_config, get_simple_connect_token, has_resource};
use crate::messaging::client::{deserialize, serialize};
use crate::messaging::NetworkEntities;

pub fn connect_to_server(server_addr: SocketAddr) -> Client {
    let bind_addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let socket = UdpSocket::bind(bind_addr).unwrap();
    let user_id = current_time.as_millis() as u64;
    let client = renet::RenetClient::new(
        current_time,
        socket,
        get_renet_connection_config(),
        ClientAuthentication::Secure {
            connect_token: get_simple_connect_token(user_id, vec![server_addr.into()])
        }
    );
    info!("Connecting to server at {}", server_addr);
    Client(client.unwrap())
}

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        use bevy::prelude::CoreStage::*;
        crate::messaging::add_events(app);

        app
            .init_resource::<NetworkEntities>()
            .add_system_to_stage(PreUpdate, update_system
                .with_run_criteria(has_resource::<Client>)
            )
            .add_system_to_stage(PreUpdate, read_packets_system
                .after(update_system)
                .with_run_criteria(has_resource::<Client>)
            )
            .add_system_to_stage(PostUpdate, detect_shutdown_system
                .before(write_packets_system)
                .with_run_criteria(has_resource::<Client>)
            )
            .add_system_to_stage(PostUpdate, write_packets_system
                .with_run_criteria(has_resource::<Client>)
            )
            .add_system_to_stage(PostUpdate, send_packets_system
                .after(write_packets_system)
                .with_run_criteria(has_resource::<Client>)
            );
    }
}

crate::make_wrapper_struct!(Client, RenetClient);

pub fn update_system(
    mut client: ResMut<Client>,
    time: Res<Time>,
) {
    if let Err(e) = client.update(time.delta()) {
        error!("Renet Update: {}", e);
    }
}

pub fn read_packets_system(world: &mut World) {
    world.resource_scope::<Client, _>(|world, mut client| {
        for channel in Channel::ALL {
            while let Some(bytes) = client.receive_message(channel) {
                deserialize(world, bytes);
            }
        }
    });
}

pub fn write_packets_system(world: &mut World) {
    world.resource_scope::<Client, _>(|world, mut client: Mut<Client>| {
        serialize(world, client.as_mut());
    })
}

pub fn send_packets_system(mut client: ResMut<Client>) {
    if let Err(e) = client.send_packets() {
        error!("Renet Send: {}", e);
    }
}

// not perfect but it'll do
fn detect_shutdown_system(mut client: ResMut<Client>, mut bevy_shutdown: EventReader<AppExit>) {
    for _ in bevy_shutdown.iter() {
        info!("Shutting down client");
        client.disconnect();
    }
}
