use crate::render::RenderState;
use crate::services::chunk_service::chunk::{Chunk, ChunkData, Chunks};
use crate::services::chunk_service::ChunkService;
use crate::services::networking_service::NetworkingService;
use crate::services::settings_service::SettingsService;
use nalgebra::Vector3;
use rc_network::protocol::packet::Packet;
use rc_network::protocol::types::PVarType;
use specs::{Read, System, Write};

pub struct ReceivedNetworkPackets {
    pub(crate) packets: Vec<Packet>,
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
        Write<'a, Chunks>,
        Read<'a, RenderState>,
        Read<'a, SettingsService>,
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
        ): Self::SystemData,
    ) {
        network_packets.packets = networking_service.get_packets();

        if network_packets.packets.len() == 0 {
            return;
        }

        for packet in network_packets.packets.iter() {
            if packet.id == 0x22 {
                let chunk_x = match packet.tokens.get(0).unwrap() {
                    PVarType::Int(val) => val,
                    _ => panic!(),
                };
                let chunk_z = match packet.tokens.get(1).unwrap() {
                    PVarType::Int(val) => val,
                    _ => panic!(),
                };

                let chunk_details = match packet.tokens.get(6).unwrap() {
                    PVarType::ChunkData(val) => val.get(0).unwrap(),
                    _ => panic!(),
                };

                let blocks = chunk_service.blocks.clone();
                chunk_service.load_chunk(
                    Some((chunk_details.data.clone(), blocks)),
                    Vector3::new(chunk_x.clone() as i32, 0, chunk_z.clone() as i32),
                    &mut chunks,
                );

                let chunk_pos = chunk_service
                    .viewable_chunks
                    .get(chunk_service.viewable_chunks.len() - 1)
                    .unwrap();

                let chunk = chunks.0.get_mut(chunk_pos).unwrap();

                unsafe {
                    if let Chunk::Tangible(chunk) = chunk {
                        let const_ptr = chunk as *const ChunkData;
                        let mut_ptr = const_ptr as *mut ChunkData;
                        let chunk = &mut *mut_ptr;

                        chunk.generate_mesh(&chunks, &settings);
                        chunk.create_buffers(
                            &render_system.device,
                            &chunk_service.model_bind_group_layout,
                        );
                    }
                }
                println!(
                    "Added chunk: {}",
                    Vector3::new(chunk_x.clone() as i32, 0, chunk_z.clone() as i32)
                );
            }
        }
    }
}
