use std::sync::atomic::Ordering;
use crate::events::authorization::AuthorizationEvent;
use crate::game::transform::Transform;
use crate::resources::{WorldData, ENTITY_ID_COUNT};
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::{Commands, EventWriter};
use bevy_ecs::system::Query;
use bevy_log::info;

use rc_client::rc_protocol::constants::{EntityId, UserId};
use rc_client::rc_protocol::protocol::clientbound::chunk_update::FullChunkUpdate;
use rc_client::rc_protocol::protocol::clientbound::ping::Ping;
use rc_client::rc_protocol::protocol::clientbound::spawn_entity::SpawnEntity;
use rc_client::rc_protocol::protocol::serverbound::pong::Pong;
use rc_client::rc_protocol::protocol::Protocol;
use rc_client::rc_protocol::types::SendPacket;
use crate::TransportSystem;

/// A user who is yet to be authorized
pub struct GameUser {
    pub name: Option<String>,

    pub last_ping: Ping,
    pub last_pong: Pong,

    pub user_id: UserId,
    pub entity_id: EntityId,

    /* If the user has been disconnected */
    pub disconnected: bool, /* Todo: Encryption */
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

        let entity = commands.spawn().insert(transform).id();
        global.entities.insert(entity_id, entity);

        // Send world to client
        for (loc, chunk) in global.chunks.iter() {
            let chunk = Protocol::PartialChunkUpdate(FullChunkUpdate::new(
                chunk.world,
                loc.x,
                loc.y,
                loc.z,
            ));

            send_packet.send(SendPacket(chunk, client.client));
        }
    }
}