use crate::events::authorization::AuthorizationEvent;
use crate::resources::{World, ENTITY_ID_COUNT};
use crate::{ReceivePacket, SendPacket, TransportSystem};
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::{EventWriter, Res};
use bevy_log::info;
use crossbeam::channel::{Receiver, Sender};
use nalgebra::Vector3;
use rustcraft_protocol::constants::{EntityId, UserId};
use rustcraft_protocol::protocol::clientbound::chunk_update::PartialChunkUpdate;
use rustcraft_protocol::protocol::clientbound::ping::Ping;
use rustcraft_protocol::protocol::clientbound::spawn_entity::SpawnEntity;
use rustcraft_protocol::protocol::serverbound::pong::Pong;
use rustcraft_protocol::protocol::Protocol;
use rustcraft_protocol::stream::GameStream;
use std::sync::atomic::Ordering;
use std::time::SystemTime;
use tokio::net::TcpStream;
use tokio::task::JoinHandle;

/// A user who is yet to be authorized
pub struct GameUser {
    pub name: Option<String>,

    pub read_packets: Receiver<ReceivePacket>,
    pub write_packets: Sender<SendPacket>,

    pub read_packet_handle: JoinHandle<()>,
    pub write_packet_handle: JoinHandle<()>,

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
    mut global: ResMut<World>,
    mut transport: ResMut<TransportSystem>,
    mut send_packet: EventWriter<SendPacket>,
) {
    for client in event_reader.iter() {
        info!("Authorisation event");
        // Spawn other entities
        for (id, entity) in &global.entities {
            let packet = Protocol::SpawnEntity(SpawnEntity {
                id: *id,
                loc: [entity.x, entity.y, entity.z],
                rot: [0.0; 4],
            });
            send_packet.send(SendPacket(packet, client.client));
        }

        // Create new entity for player
        let entity_id = EntityId(ENTITY_ID_COUNT.fetch_add(1, Ordering::Acquire));
        let entity = Vector3::zeros();
        global.entities.insert(entity_id, entity);

        // Store player entity
        transport.clients.get_mut(&client.client).unwrap().entity_id = entity_id;

        // Spawn new player for other players
        for (id, user) in &transport.clients {
            let entity = global.entities.get(&user.entity_id).unwrap();
            let packet = Protocol::SpawnEntity(SpawnEntity {
                id: entity_id,
                loc: [entity.x, entity.y, entity.z],
                rot: [0.0; 4],
            });
            send_packet.send(SendPacket(packet, *id));
        }

        // Send world to client
        for (loc, chunk) in global.chunks.iter() {
            let chunk = Protocol::PartialChunkUpdate(PartialChunkUpdate::new(
                chunk.world,
                loc.x,
                loc.y,
                loc.z,
            ));

            send_packet.send(SendPacket(chunk, client.client));
        }
    }
}
