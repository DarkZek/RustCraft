use crate::game::chunk::ChunkData;
use crate::game::transform::Transform;
use crate::helpers::global_to_local_position;
use crate::{info, ReceivePacket, SendPacket, TransportSystem, World};
use bevy_ecs::event::{EventReader, EventWriter};
use bevy_ecs::prelude::{Query, Res};
use bevy_ecs::system::ResMut;
use nalgebra::{Quaternion, Vector3};
use rustcraft_protocol::constants::CHUNK_SIZE;
use rustcraft_protocol::protocol::clientbound::block_update::BlockUpdate;
use rustcraft_protocol::protocol::clientbound::entity_moved::EntityMoved;
use rustcraft_protocol::protocol::clientbound::entity_rotated::EntityRotated;
use rustcraft_protocol::protocol::Protocol;

pub fn receive_message_event(
    mut event_reader: EventReader<ReceivePacket>,
    mut event_writer: EventWriter<SendPacket>,
    mut global: ResMut<World>,
    system: Res<TransportSystem>,
    mut transforms: Query<&mut Transform>,
) {
    for event in event_reader.iter() {
        match &event.0 {
            Protocol::PlayerMove(packet) => {
                // Update all other clients
                let entity = system.clients.get(&event.1).unwrap().entity_id;

                // TODO: Don't trust user input

                let send_packet = Protocol::EntityMoved(EntityMoved {
                    entity,
                    x: packet.x,
                    y: packet.y,
                    z: packet.z,
                });

                for (client, _) in &system.clients {
                    if *client == event.1 {
                        continue;
                    }
                    info!("Move packet sending to {:?}", client);
                    event_writer.send(SendPacket(send_packet.clone(), *client));
                }

                if let Some(val) = global.entities.get(&entity) {
                    // Move player in ecs
                    transforms.get_mut(*val).unwrap().position =
                        Vector3::new(packet.x, packet.y, packet.z);
                }
            }
            Protocol::PlayerRotate(packet) => {
                // Update all other clients
                let entity = system.clients.get(&event.1).unwrap().entity_id;

                // TODO: Don't trust user input

                let send_packet = Protocol::EntityRotated(EntityRotated {
                    entity,
                    x: packet.x,
                    y: packet.y,
                    z: packet.z,
                    w: packet.w,
                });

                for (client, _) in &system.clients {
                    if *client == event.1 {
                        continue;
                    }
                    //info!("Move packet sent to {:?}", client);
                    event_writer.send(SendPacket(send_packet.clone(), *client));
                }

                if let Some(val) = global.entities.get(&entity) {
                    // Rotate player in ecs
                    transforms.get_mut(*val).unwrap().rotation =
                        Quaternion::new(packet.x, packet.y, packet.z, packet.w);
                }
            }
            Protocol::BlockUpdate(packet) => {
                // TODO: Don't trust user input

                let packet = BlockUpdate::new(packet.id, packet.x, packet.y, packet.z);

                for (client, _) in &system.clients {
                    if *client == event.1 {
                        continue;
                    }
                    info!("Block update packet sent to {:?}", client);
                    event_writer.send(SendPacket(Protocol::BlockUpdate(packet), *client));
                }

                let (chunk_loc, inner_loc) =
                    global_to_local_position(Vector3::new(packet.x, packet.y, packet.z));

                // Store
                if let Some(mut chunk) = global.chunks.get_mut(&chunk_loc) {
                    // Found chunk! Update block
                    chunk.world[inner_loc.x][inner_loc.y][inner_loc.z] = 1;
                } else {
                    // Create chunk data
                    let mut chunk = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

                    // Set block
                    chunk[inner_loc.x][inner_loc.y][inner_loc.z] = 1;

                    // Create chunk
                    global
                        .chunks
                        .insert(chunk_loc, ChunkData::new(chunk_loc, chunk));
                }
            }
            _ => {}
        }
    }
}
