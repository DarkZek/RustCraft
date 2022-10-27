use crate::services::asset::AssetService;
use crate::services::networking::transport::packet::ReceivePacket;
use crate::{
    default, error, info, shape, Assets, ChunkData, Color, Commands, Entity, EventWriter, Mesh,
    Mut, PbrBundle, Quat, Query, RerenderChunkFlag, ResMut, StandardMaterial, Vec3, World,
};
use crate::{EventReader, Transform};
use bevy::render::primitives::Aabb;
use nalgebra::Vector3;
use rustcraft_protocol::protocol::clientbound::chunk_update::PartialChunkUpdate;
use rustcraft_protocol::protocol::clientbound::player_join::PlayerJoin;
use rustcraft_protocol::protocol::serverbound::authenticate::UserAuthenticate;
use rustcraft_protocol::protocol::Protocol;
use std::collections::HashMap;

pub fn messages_update(
    mut event_reader: EventReader<ReceivePacket>,
    mut transforms: Query<&mut Transform>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in event_reader.iter() {
        match &event.0 {
            Protocol::EntityMoved(update) => {
                info!("Received Move Event");
                // if let Some(Ok(mut transform)) = update
                //     .entity
                //     .get(&client)
                //     .map(|client| transforms.get_mut(client))
                // {
                //     transform.translation.x = *update.x;
                //     transform.translation.y = *update.y;
                //     transform.translation.z = *update.z;
                // } else {
                //     error!("Move event recieved before entity created");
                // }
            }
            Protocol::EntityRotated(update) => {
                // if let Some(Ok(mut transform)) = update
                //     .player
                //     .get(&client)
                //     .map(|client| transforms.get_mut(client))
                // {
                //     transform.rotation =
                //         Quat::from_xyzw(*update.x, *update.y, *update.z, *update.w);
                // } else {
                //     error!("Rotate event recieved before entity created");
                // }
            }
            Protocol::PlayerJoin(join) => {
                info!("Player spawned {:?}!", join.username);
                // if let Some(entity) = join.entity.get(&client) {
                //     commands.entity(entity).insert_bundle(PbrBundle {
                //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                //         material: materials.add(Color::rgb(0.3, 0.8, 0.3).into()),
                //         ..default()
                //     });
                // } else {
                //     info!("Entity has not spawned yet!");
                // }
            }
            Protocol::PartialChunkUpdate(update) => {}
            t => {
                info!("Other {:?}", t);
            }
        }
    }
}
