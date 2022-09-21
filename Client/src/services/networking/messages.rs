use crate::services::asset::AssetService;
use crate::services::chunk::data::partial::PartialChunks;
use crate::{
    default, error, info, shape, Assets, ChunkData, Color, Commands, Entity, EventWriter, Mesh,
    Mut, PbrBundle, Quat, Query, RerenderChunkFlag, ResMut, StandardMaterial, Vec3, World,
};
use crate::{EventReader, Transform};
use bevy::render::primitives::Aabb;
use bevy_testing_protocol::channels::Channels;
use bevy_testing_protocol::protocol::clientbound::chunk_update::PartialChunkUpdate;
use bevy_testing_protocol::protocol::clientbound::player_join::PlayerJoin;
use bevy_testing_protocol::protocol::serverbound::authenticate::UserAuthenticate;
use bevy_testing_protocol::protocol::Protocol;
use naia_bevy_client::events::MessageEvent;
use naia_bevy_client::events::SpawnEntityEvent;
use naia_bevy_client::{Client, CommandHistory};
use nalgebra::Vector3;
use std::collections::HashMap;

pub fn messages_update(
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut client: Client<Protocol, Channels>,
    mut transforms: Query<&mut Transform>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in event_reader.iter() {
        match event {
            MessageEvent(Channels::StatusUpdate, Protocol::PlayerMoved(update)) => {
                info!("Received Move Event");
                if let Some(Ok(mut transform)) = update
                    .player
                    .get(&client)
                    .map(|client| transforms.get_mut(client))
                {
                    transform.translation.x = *update.x;
                    transform.translation.y = *update.y;
                    transform.translation.z = *update.z;
                } else {
                    error!("Move event recieved before entity created");
                }
            }
            MessageEvent(Channels::StatusUpdate, Protocol::PlayerRotated(update)) => {
                if let Some(Ok(mut transform)) = update
                    .player
                    .get(&client)
                    .map(|client| transforms.get_mut(client))
                {
                    transform.rotation =
                        Quat::from_xyzw(*update.x, *update.y, *update.z, *update.w);
                } else {
                    error!("Rotate event recieved before entity created");
                }
            }
            MessageEvent(Channels::StatusUpdate, Protocol::PlayerJoin(join)) => {
                if let Some(entity) = join.entity.get(&client) {
                    info!("Player spawned {}!", *join.username);
                    commands.entity(entity).insert_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(Color::rgb(0.3, 0.8, 0.3).into()),
                        ..default()
                    });
                } else {
                    info!("Entity has not spawned yet!");
                }
            }
            MessageEvent(Channels::ChunkUpdates, Protocol::PartialChunkUpdate(update)) => { todo!() }
            _ => {
                info!("Other");
            }
        }
    }
}
