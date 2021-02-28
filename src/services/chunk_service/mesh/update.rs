use crate::services::chunk_service::chunk::{ChunkData, Chunks};
use crate::services::chunk_service::mesh::rerendering::UpdateChunkMesh;
use specs::Join;
use specs::{System, WriteStorage};

pub struct ChunkMeshUpdateSystem;

impl<'a> System<'a> for ChunkMeshUpdateSystem {
    type SystemData = (
        WriteStorage<'a, UpdateChunkMesh>,
        WriteStorage<'a, ChunkData>,
    );

    fn run(&mut self, (mut flags, mut chunks): Self::SystemData) {
        let mut chunks_loc = Chunks::new_mut((&mut chunks).join().collect::<Vec<&mut ChunkData>>());

        for flag in flags.drain().join() {
            if let Option::Some(chunk) = chunks_loc.get_mut_loc(flag.chunk) {
                chunk.set_mesh(flag);
            }
        }
    }
}

impl ChunkData {
    pub fn set_mesh(&mut self, data: UpdateChunkMesh) {
        match data {
            UpdateChunkMesh {
                chunk,
                opaque_model,
                translucent_model,
                viewable_map,
                model_bind_group,
            } => {
                self.opaque_model = opaque_model;
                self.translucent_model = translucent_model;
                self.model_bind_group = model_bind_group;
                self.viewable_map = viewable_map;
            }
        }
    }
}
