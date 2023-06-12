use crate::events::authorization::AuthorizationEvent;
use crate::game::transform::Transform;
use bevy::ecs::change_detection::ResMut;
use bevy::ecs::event::EventReader;
use bevy::ecs::prelude::{Commands, EventWriter};
use bevy::ecs::system::Query;
use bevy::log::info;
use bevy::prelude::Entity;
use nalgebra::Vector3;
use std::sync::atomic::Ordering;

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
        info!("Authorisation event");
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
            send_packet.send(SendPacket(packet, client.client));
        }

        let transform = Transform::default();

        // Create new entity for player
        let entity_id = EntityId(ENTITY_ID_COUNT.fetch_add(1, Ordering::Acquire));

        // Store player entity
        transport.clients.get_mut(&client.client).unwrap().entity_id = entity_id;

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
            if *id == client.client {
                continue;
            }
            send_packet.send(SendPacket(packet.clone(), *id));
        }

        let player_pos = transform.position.clone();

        let entity = commands.spawn(transform).id();
        global.entities.insert(entity_id, entity.clone());
        transport.clients.get_mut(&client.client).unwrap().entity = Some(entity);

        let mut chunks = global.chunks.keys();

        // Send world to client, sorting by closest chunks
        for loc in chunks {
            chunk_system
                .requesting_chunks
                .entry(client.client)
                .or_insert_with(|| vec![])
                .push(*loc);
        }
    }
}
