use crate::entity::player::PlayerEntity;
use crate::game::physics::PhysicsObject;
use crate::render::RenderState;
use crate::services::chunk_service::chunk::{ChunkData, Chunks};
use crate::services::chunk_service::ChunkService;
use crate::services::networking_service::NetworkingService;
use crate::services::settings_service::SettingsService;
use nalgebra::Vector3;
use rc_network::protocol::packet::PacketData;

use crate::game::game_state::{GameState, ProgramState};
use crate::helpers::chunk_by_loc_from_write;
use crate::services::chunk_service::mesh::rerendering::{RerenderChunkFlag, UpdateChunkMesh};
use crate::services::ui_service::UIService;
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
        WriteStorage<'a, PhysicsObject>,
        ReadStorage<'a, PlayerEntity>,
        Entities<'a>,
        WriteStorage<'a, RerenderChunkFlag>,
        Write<'a, GameState>,
        Write<'a, UIService>,
    );

    fn run(
        &mut self,
        (
            mut network_packets,
            mut networking_service,
            mut chunk_service,
            mut chunks_storage,
            mut player_physics,
            player_entity,
            mut entities,
            mut rerendering_chunks,
            mut game_state,
            mut ui_service,
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

            if let PacketData::PlayerListInfo(packet) = &packet {
                game_state.state = ProgramState::IN_GAME;
                let image = ui_service.background_image.clone();
                ui_service.images.delete_image(image);

                log!("Successfully logged in to server");
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
                        Some(packet.data.get(chunks_index).unwrap().data.clone()),
                        Vector3::new(packet.x, y, packet.z),
                        &mut entities,
                        &mut chunks_storage,
                        &mut rerendering_chunks,
                    );

                    let chunk = Vector3::new(packet.x, y, packet.z);

                    entities
                        .build_entity()
                        .with(RerenderChunkFlag { chunk }, &mut rerendering_chunks)
                        .build();

                    chunks_index += 1;
                }
            }
        }
    }
}
