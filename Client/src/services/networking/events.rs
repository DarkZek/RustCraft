use crate::{
    info, Color, Commands, EventReader, Query, ResMut, Sprite, SpriteBundle, Transform, Vec2,
};
use bevy_testing_protocol::channels::Channels;
use bevy_testing_protocol::protocol::{Protocol, ProtocolKind};
use naia_bevy_client::events::{InsertComponentEvent, SpawnEntityEvent, UpdateComponentEvent};
use naia_bevy_client::shared::{sequence_greater_than, Tick};
use naia_bevy_client::Client;

pub fn connect_event(client: Client<Protocol, Channels>) {
    info!("Client connected to: {}", client.server_address());
}

pub fn disconnect_event(client: Client<Protocol, Channels>) {
    info!("Client disconnected from: {}", client.server_address());
}

pub fn spawn_entity_event(mut event_reader: EventReader<SpawnEntityEvent>) {
    for event in event_reader.iter() {
        match event {
            SpawnEntityEvent(entity) => {
                info!("spawned entity");
            }
        }
    }
}
