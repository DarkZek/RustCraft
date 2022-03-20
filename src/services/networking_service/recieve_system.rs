use crate::entity::player::PlayerEntity;
use crate::game::physics::PhysicsObject;
use crate::services::chunk_service::chunk::{ChunkData, ChunkEntityLookup};
use crate::services::chunk_service::ChunkService;
use crate::services::networking_service::NetworkingService;
use nalgebra::Vector3;
use rc_network::protocol::packet::clientbound::ClientBoundPacketData;

use crate::game::game_state::{GameState, ProgramState};
use crate::services::chunk_service::mesh::rerendering::RerenderChunkFlag;
use crate::services::settings_service::CHUNK_SIZE;
use crate::services::ui_service::UIService;
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};

pub struct ReceivedNetworkPackets {
    pub(crate) packets: Vec<ClientBoundPacketData>,
}

impl Default for ReceivedNetworkPackets {
    fn default() -> Self {
        ReceivedNetworkPackets { packets: vec![] }
    }
}

pub struct NetworkingRecieveSystem;

impl<'a> System<'a> for NetworkingRecieveSystem {
    type SystemData = (
        Write<'a, ReceivedNetworkPackets>,
        Write<'a, NetworkingService>,
        Write<'a, ChunkService>,
        WriteStorage<'a, ChunkData>,
        WriteStorage<'a, PhysicsObject>,
        Read<'a, PlayerEntity>,
        Entities<'a>,
        WriteStorage<'a, RerenderChunkFlag>,
        Write<'a, GameState>,
        Write<'a, UIService>,
        Write<'a, ChunkEntityLookup>,
    );

    fn run(
        &mut self,
        (
            mut network_packets,
            mut networking_service,
            mut chunk_service,
            mut chunks_storage,
            mut physics_objects,
            player_entity,
            mut entities,
            mut rerendering_chunks,
            mut game_state,
            mut ui_service,
            mut chunk_entity_lookup,
        ): Self::SystemData,
    ) {
        network_packets.packets = networking_service.get_packets();

        if network_packets.packets.len() == 0 {
            return;
        }

        for packet in network_packets.packets.iter() {
            if let ClientBoundPacketData::SpawnPlayer(packet) = &packet {
                let player_physics = physics_objects.get_mut(player_entity.0).unwrap();

                player_physics.position =
                    Vector3::new(packet.x as f32, packet.y as f32 + 100.0, packet.z as f32);
                log!("Spawned player!!");
            }

            // Handle block changes
            if let ClientBoundPacketData::MultiBlockChange(packet) = packet {
                let mut chunks = Vec::new();

                let mut modified_chunks = Vec::new();

                for chunk in (&mut chunks_storage).join() {
                    if chunk.position.x == packet.x && chunk.position.z == packet.z {
                        chunks.push(chunk);
                    }
                }

                for (x, y, z, block_id) in &packet.changes {
                    let y_chunk = (*y as f32 / CHUNK_SIZE as f32).floor() as i32;
                    let y_local = y % CHUNK_SIZE as u8;
                    let mut chunk = None;

                    for c in &mut chunks {
                        if c.position.y == y_chunk {
                            chunk = Some(c);
                            break;
                        }
                    }

                    match chunk {
                        Some(val) => {
                            val.world[*x as usize][y_local as usize][*z as usize] =
                                *block_id as u32;

                            if !modified_chunks.contains(&Vector3::new(packet.x, y_chunk, packet.z))
                            {
                                modified_chunks.push(Vector3::new(packet.x, y_chunk, packet.z))
                            }
                        }
                        None => {
                            log_error!(
                                "Network tried to change block in unloaded chunk X: {} Z: {}",
                                packet.x,
                                packet.z
                            );
                        }
                    }
                }

                // Schedule for rebuild
                for chunk in modified_chunks {
                    entities
                        .build_entity()
                        .with(RerenderChunkFlag { chunk }, &mut rerendering_chunks)
                        .build();
                }
            }

            if let ClientBoundPacketData::ChunkData(packet) = packet {
                let mut mask = packet.primary_bit_mask.clone();
                let mut chunks_index = 0;

                for y in 0..8 {
                    if mask & 0b1 == 0 {
                        mask >>= 0b1;
                        continue;
                    }
                    mask >>= 0b1;

                    chunk_service.load_chunk(
                        Some(packet.data.get(chunks_index).unwrap().data.clone()),
                        Vector3::new(packet.x, y, packet.z),
                        &mut entities,
                        &mut chunks_storage,
                        &mut rerendering_chunks,
                        &mut chunk_entity_lookup,
                    );

                    let chunk = Vector3::new(packet.x, y, packet.z);

                    entities
                        .build_entity()
                        .with(RerenderChunkFlag { chunk }, &mut rerendering_chunks)
                        .build();

                    chunks_index += 1;
                }
            }

            if let ClientBoundPacketData::LoginSuccess(packet) = packet {
                game_state.state = ProgramState::InGame;
                let image = ui_service.background_image.clone();
                ui_service.images.delete_image(image);

                log!("Successfully logged in to server");
            }

            if let ClientBoundPacketData::PlayerPositionLook(packet) = packet {
                let player_physics = physics_objects.get_mut(player_entity.0).unwrap();

                let mut pos = player_physics.position;

                if packet.flags & 0x1 != 0 {
                    pos.x += packet.x as f32;
                } else {
                    pos.x = packet.x as f32;
                }
                if packet.flags & 0x2 != 0 {
                    pos.y += packet.y as f32;
                } else {
                    pos.y = packet.y as f32;
                }
                if packet.flags & 0x4 != 0 {
                    pos.z += packet.z as f32;
                } else {
                    pos.z = packet.z as f32;
                }

                game_state.player.rot = [packet.yaw, packet.pitch];

                println!("Recieved position {:?}", packet);

                println!("{:?}", pos);

                player_physics.new_position = pos;
            }
        }
    }
}
