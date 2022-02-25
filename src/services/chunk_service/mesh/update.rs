use crate::helpers::lerp_color;
use crate::services::chunk_service::chunk::{ChunkData, Chunks, RawLightingData};
use crate::services::chunk_service::lighting::UpdateChunkLighting;
use crate::services::chunk_service::mesh::rerendering::{UpdateChunkGraphics, UpdateChunkMesh};
use crate::services::chunk_service::ChunkService;
use specs::{Join, Write};
use specs::{System, WriteStorage};

pub struct ChunkMeshUpdateSystem;

impl<'a> System<'a> for ChunkMeshUpdateSystem {
    type SystemData = (
        WriteStorage<'a, UpdateChunkGraphics>,
        WriteStorage<'a, ChunkData>,
        Write<'a, ChunkService>,
    );

    fn run(&mut self, (mut flags, mut chunks, mut chunk_service): Self::SystemData) {
        let mut chunks_loc = Chunks::new_mut((&mut chunks).join().collect::<Vec<&mut ChunkData>>());

        for flag in flags.drain().join() {
            if let Option::Some(selected_chunk) = chunks_loc.get_mut_loc(flag.mesh.chunk) {
                let UpdateChunkGraphics { mesh, lighting } = flag;

                selected_chunk.set_mesh(mesh);

                let UpdateChunkLighting { chunk, adjacent } = lighting;
                selected_chunk.set_lighting(chunk);

                for (coords, lighting) in adjacent {
                    if let Some(chunk) = chunks_loc.get_mut_loc(coords) {
                        chunk.add_adjacent_lighting(lighting);
                    }
                }
            }
        }

        // Update visible chunks
        chunk_service.update_culling = true;
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

    pub fn set_lighting(&mut self, data: RawLightingData) {
        self.light_levels = data;
    }

    pub fn add_adjacent_lighting(&mut self, data: RawLightingData) {
        for x in 0..self.neighboring_light_levels.len() {
            for y in 0..self.neighboring_light_levels.len() {
                for z in 0..self.neighboring_light_levels.len() {
                    let color =
                        lerp_color(self.neighboring_light_levels[x][y][z], data[x][y][z], 0.5);
                    self.neighboring_light_levels[x][y][z] = color;
                }
            }
        }
    }
}
