use crate::entity::player::PlayerEntity;
use crate::game::physics::PhysicsObject;
use crate::services::chunk_service::chunk::ChunkData;
use crate::services::chunk_service::ChunkService;
use crate::services::networking_service::NetworkingService;
use nalgebra::Vector3;
use rc_network::protocol::packet::PacketData;

use crate::game::game_state::{GameState, ProgramState};
use crate::services::chunk_service::mesh::rerendering::RerenderChunkFlag;
use crate::services::ui_service::UIService;
use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};

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
                    Vector3::new(packet.x as f32, packet.y as f32 + 100.0, packet.z as f32);
                log!("Spawned player!!");
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

                    // Loaded enough to show game
                    if chunks_storage.as_slice().len() >= 16 * 16 * 16
                        && game_state.state != ProgramState::IN_GAME
                    {
                        game_state.state = ProgramState::IN_GAME;
                        let image = ui_service.background_image.clone();
                        ui_service.images.delete_image(image);

                        log!("Successfully logged in to server");
                    }

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
