use crate::game::transform::Transform;
use bevy::ecs::change_detection::ResMut;
use bevy::ecs::event::EventReader;
use bevy::ecs::prelude::{Commands, EventWriter};
use bevy::ecs::system::Query;
use bevy::log::info;
use bevy::prelude::Entity;
use nalgebra::Vector3;
use std::sync::atomic::Ordering;

use crate::events::authorize::AuthorizationEvent;
use crate::game::world::data::ENTITY_ID_COUNT;
use crate::helpers::global_to_local_position;
use crate::systems::chunk::ChunkSystem;
use crate::{TransportSystem, WorldData};
use rc_networking::constants::{EntityId, UserId, CHUNK_SIZE};
use rc_networking::protocol::clientbound::chunk_update::FullChunkUpdate;
use rc_networking::protocol::clientbound::spawn_entity::SpawnEntity;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;

/// A user who is yet to be authorized
pub struct GameUser {
    pub name: Option<String>,

    pub user_id: UserId,
    pub entity_id: EntityId,
    pub entity: Option<Entity>,
    pub loading: bool,
}

impl GameUser {
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
}

pub fn authorization_event(
    mut event_reader: EventReader<AuthorizationEvent>,
    mut global: ResMut<WorldData>,
    mut transport: ResMut<TransportSystem>,
    mut send_packet: EventWriter<SendPacket>,
    mut commands: Commands,
    transforms: Query<&Transform>,
    mut chunk_system: ResMut<ChunkSystem>,
) {
    for client in event_reader.iter() {
        info!("Player {:?} logged in. Sending chunks.", client.user_id);

        // Spawn other entities for new player
        for (id, entity) in &global.entities {
            let transform = transforms.get(*entity).unwrap();
            let packet = Protocol::SpawnEntity(SpawnEntity {
                id: *id,
                loc: [
                    transform.position.x,
                    transform.position.y,
                    transform.position.z,
                ],
                rot: transform.rotation.coords.into(),
            });
            send_packet.send(SendPacket(packet, client.user_id));
        }

        let transform = Transform::default();

        // Create new entity for player
        let entity_id = EntityId(ENTITY_ID_COUNT.fetch_add(1, Ordering::Acquire));

        // Store player entity
        transport
            .clients
            .get_mut(&client.user_id)
            .unwrap()
            .entity_id = entity_id;

        let packet = Protocol::SpawnEntity(SpawnEntity {
            id: entity_id,
            loc: [
                transform.position.x,
                transform.position.y,
                transform.position.z,
            ],
            rot: [0.0; 4],
        });

        // Spawn new player for other players
        for (id, _) in &transport.clients {
            // Don't spawn new client for itself
            if *id == client.user_id {
                continue;
            }
            send_packet.send(SendPacket(packet.clone(), *id));
        }

        let player_pos = transform.position.clone();

        let entity = commands.spawn(transform).id();
        global.entities.insert(entity_id, entity.clone());
        transport.clients.get_mut(&client.user_id).unwrap().entity = Some(entity);

        let mut chunks = global.chunks.keys();

        let (player_chunk, _) = global_to_local_position(Vector3::new(
            player_pos.x as i32,
            player_pos.y as i32,
            player_pos.z as i32,
        ));

        let chunk_load_radius = 3;

        // Load chunks around player
        for x in (player_chunk.x - chunk_load_radius)..(player_chunk.x + chunk_load_radius) {
            for y in (player_chunk.y - chunk_load_radius)..(player_chunk.y + chunk_load_radius) {
                for z in (player_chunk.z - chunk_load_radius)..(player_chunk.z + chunk_load_radius)
                {
                    chunk_system
                        .requesting_chunks
                        .entry(client.user_id)
                        .or_insert_with(|| vec![])
                        .push(Vector3::new(x, y, z));
                }
            }
        }

        // List this user as still loading in content, so we know to send them a packet to close the loading screen once chunks have been sent
        transport.initialising_clients.insert(client.user_id);
    }
}
