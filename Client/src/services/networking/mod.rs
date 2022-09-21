use crate::services::networking::chunk::network_chunk_sync;
use crate::services::networking::events::{connect_event, disconnect_event, spawn_entity_event};
use crate::services::networking::location_sync::{
    network_location_sync, LastNetworkRotationSync, LastNetworkTranslationSync,
};
use crate::services::networking::messages::messages_update;
use crate::{info, services, App, PartialChunks, Plugin, Quat};
use bevy::prelude::Vec3;
use bevy_testing_protocol::channels::Channels;
use bevy_testing_protocol::config::network_config;
use bevy_testing_protocol::protocol::serverbound::authenticate::UserAuthenticate;
use bevy_testing_protocol::protocol::Protocol;
use naia_bevy_client::{Client, Stage};
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin};
use nalgebra::Vector3;

mod chunk;
mod events;
mod location_sync;
mod messages;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ClientPlugin::<Protocol, Channels>::new(
            ClientConfig::default(),
            network_config(),
        ))
        .add_startup_system(server_connect)
        .add_system_to_stage(Stage::Connection, connect_event)
        .add_system_to_stage(Stage::Disconnection, disconnect_event)
        .add_system_to_stage(Stage::ReceiveEvents, messages_update)
        .add_system_to_stage(Stage::ReceiveEvents, spawn_entity_event)
        .add_system(network_location_sync)
        .add_system(network_chunk_sync)
        .insert_resource(PartialChunks::default())
        .insert_resource(LastNetworkTranslationSync(Vec3::default()))
        .insert_resource(LastNetworkRotationSync(Quat::default()));
    }
}

pub fn server_connect(mut client: Client<Protocol, Channels>) {
    client.auth(UserAuthenticate::new("chaie"));
    client.connect("http://127.0.0.1:14191");
    info!("Connecting");
}
