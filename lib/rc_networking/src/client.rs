use bevy::prelude::*;
use renet::RenetClient;
use crate::Channel;
use crate::messaging::client::{deserialize, serialize};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        todo!()
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
