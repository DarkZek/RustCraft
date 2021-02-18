use crate::render::RenderState;
use crate::services::chunk_service::chunk::{ChunkData, Chunks, RerenderChunkFlag};
use crate::services::chunk_service::ChunkService;
use crate::services::settings_service::SettingsService;
use specs::{Join, Read};
use specs::{System, WriteStorage};

pub struct ChunkRerenderSystem;

impl<'a> System<'a> for ChunkRerenderSystem {
    type SystemData = (
        WriteStorage<'a, RerenderChunkFlag>,
        WriteStorage<'a, ChunkData>,
        Read<'a, SettingsService>,
        Read<'a, ChunkService>,
        Read<'a, RenderState>,
    );

    fn run(
        &mut self,
        (mut flags, mut chunks, settings, chunk_service, render_system): Self::SystemData,
    ) {
        let mut chunks_loc = Chunks::new(chunks.as_slice());

        for flag in flags.join() {
            if let Option::Some(chunk) = chunks_loc.get_loc(flag.chunk) {
                // Unsafely upgrade reference
                let const_ptr = chunk as *const ChunkData;
                let mut_ptr = const_ptr as *mut ChunkData;
                let chunk = unsafe { &mut *mut_ptr };

                chunk.generate_mesh(&chunks, &settings);
                chunk.create_buffers(
                    &render_system.device,
                    &chunk_service.model_bind_group_layout,
                );
            }
        }

        flags.clear();
    }
}
