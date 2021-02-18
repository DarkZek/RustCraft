use crate::entity::player::PlayerEntity;
use crate::game::physics::PhysicsObject;
use crate::render::RenderState;
use crate::services::chunk_service::chunk::{ChunkData, RerenderChunkFlag};
use crate::services::chunk_service::ChunkService;
use crate::services::networking_service::NetworkingService;
use crate::services::settings_service::SettingsService;
use nalgebra::Vector3;
use rc_network::protocol::packet::PacketData;

use crate::block::blocks::BlockStates;
use crate::helpers::{chunk_by_loc_from_read, chunk_by_loc_from_write};
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};

pub struct ReceivedNetworkPackets {
    pub(crate) packets: Vec<PacketData>,
}

impl Default for ReceivedNetworkPackets {
    fn default() -> Self {
        ReceivedNetworkPackets { packets: vec![] }
    }
}

pub struct NetworkingSyncSystem;

impl<'a> System<'a> for NetworkingSyncSystem {
    type SystemData = (
        Write<'a, ReceivedNetworkPackets>,
        Write<'a, NetworkingService>,
        Write<'a, ChunkService>,
        WriteStorage<'a, ChunkData>,
        Read<'a, RenderState>,
        Read<'a, SettingsService>,
        WriteStorage<'a, PhysicsObject>,
        ReadStorage<'a, PlayerEntity>,
        Entities<'a>,
        WriteStorage<'a, RerenderChunkFlag>,
    );

    fn run(
        &mut self,
        (
            mut network_packets,
            mut networking_service,
            mut chunk_service,
            mut chunks,
            render_system,
            settings,
            mut player_physics,
            player_entity,
            mut entities,
            mut rerendering_chunks,
        ): Self::SystemData,
    ) {
        network_packets.packets = networking_service.get_packets();

        if network_packets.packets.len() == 0 {
            return;
        }

        for packet in network_packets.packets.iter() {
            if let PacketData::SpawnPlayer(packet) = &packet {
                let (_, player_physics) =
                    (&player_entity, &mut player_physics).join().last().unwrap();
                player_physics.position =
                    Vector3::new(packet.x as f32, packet.y as f32, packet.z as f32);
            }

            if let PacketData::ChunkData(packet) = packet {
                let mut mask = packet.primary_bit_mask.clone();
                let mut chunks_index = 0;

                for y in 0..8 {
                    if mask & 0b1 == 0 {
                        mask >>= 0b1;
                        continue;
                    }
                    mask >>= 0b1;

                    chunk_service.load_chunk(
                        Some((packet.data.get(chunks_index).unwrap().data.clone(), vec![])),
                        Vector3::new(packet.x, y, packet.z),
                        &mut entities,
                        &mut chunks,
                        &mut rerendering_chunks,
                    );

                    let chunk_pos = chunk_service
                        .viewable_chunks
                        .get(chunk_service.viewable_chunks.len() - 1)
                        .unwrap();

                    let chunk = chunk_by_loc_from_write(&chunks, *chunk_pos).unwrap();

                    unsafe {
                        let const_ptr = chunk as *const ChunkData;
                        let mut_ptr = const_ptr as *mut ChunkData;
                        let chunk = &mut *mut_ptr;

                        chunk.generate_mesh(&chunks, &settings);
                        chunk.create_buffers(
                            &render_system.device,
                            &chunk_service.model_bind_group_layout,
                        );
                    }

                    chunks_index += 1;
                }
            }
        }
    }
}
