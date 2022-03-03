use crate::block::blocks::BLOCK_STATES;
use crate::helpers::Lerp;
use crate::services::chunk_service::chunk::{ChunkData, Chunks};
use crate::services::chunk_service::mesh::culling::ViewableDirection;
use crate::services::chunk_service::mesh::rerendering::UpdateChunkMesh;
use crate::services::chunk_service::mesh::MeshData;
use crate::services::settings_service::SettingsService;
use nalgebra::Vector3;
use std::collections::HashMap;

// Our mesh generation system

impl ChunkData {
    pub fn generate_mesh(&self, chunks: &Chunks, settings: &SettingsService) -> UpdateChunkMesh {
        // Get adjacent chunks
        let mut map = HashMap::new();
        map.insert(
            Vector3::new(0, 1, 0),
            chunks.get_loc(self.position + Vector3::new(0, 1, 0)),
        );
        map.insert(
            Vector3::new(0, -1, 0),
            chunks.get_loc(self.position + Vector3::new(0, -1, 0)),
        );
        map.insert(
            Vector3::new(1, 0, 0),
            chunks.get_loc(self.position + Vector3::new(1, 0, 0)),
        );
        map.insert(
            Vector3::new(-1, 0, 0),
            chunks.get_loc(self.position + Vector3::new(-1, 0, 0)),
        );
        map.insert(
            Vector3::new(0, 0, 1),
            chunks.get_loc(self.position + Vector3::new(0, 0, 1)),
        );
        map.insert(
            Vector3::new(0, 0, -1),
            chunks.get_loc(self.position + Vector3::new(0, 0, -1)),
        );

        let viewable = self.generate_viewable_map(map, settings.chunk_edge_faces);

        let mut opaque_vertices = Vec::new();
        let mut opaque_indices = Vec::new();

        let mut translucent_vertices = Vec::new();
        let mut translucent_indices = Vec::new();

        // Create the buffers to add the mesh data into
        let chunk = self.world;

        for x in 0..chunk.len() {
            for z in 0..chunk[0][0].len() {
                for y in 0..chunk[0].len() {
                    let viewable = viewable[x][y][z].0;

                    //Isn't air
                    if chunk[x][y][z] != 0 && viewable != 0 {
                        let block = match BLOCK_STATES.get() {
                            None => {
                                log_error!("Blockstates list was not generated");
                                continue;
                            }
                            Some(states) => match states.get_block(chunk[x][y][z] as usize) {
                                None => {
                                    // TEMP commented out
                                    // log_error!(format!(
                                    //     "Block with invalid blockstate: X {} Y {} Z {} Block ID {}",
                                    // x, y, z, chunk[x][y][z]
                                    // ));
                                    continue;
                                }
                                Some(block) => block,
                            },
                        };

                        let light_color = self.light_levels[x][y][z];
                        let extra_light_color = self.neighboring_light_levels[x][y][z].clone();

                        let lightness = light_color[3].max(extra_light_color[3]);
                        let lightness_ratio = match (light_color[3], extra_light_color[3]) {
                            (0, _) => 0.0,
                            (_, 0) => 1.0,
                            (_, _) => {
                                extra_light_color[3] as f32
                                    / (light_color[3] as f32 + extra_light_color[3] as f32)
                            }
                        };

                        //TODO: Re-enable lighting
                        let _out_color = [
                            light_color[0].lerp(extra_light_color[0], lightness_ratio),
                            light_color[1].lerp(extra_light_color[1], lightness_ratio),
                            light_color[2].lerp(extra_light_color[2], lightness_ratio),
                            lightness,
                        ];

                        let vertices = if block.block_type.get_transparency()
                            || chunk[x][y][z] == 34
                            || chunk[x][y][z] == 230
                        {
                            &mut translucent_vertices
                        } else {
                            &mut opaque_vertices
                        };

                        // Change buffer based on transparency
                        let indices = if block.block_type.get_transparency()
                            || chunk[x][y][z] == 34
                            || chunk[x][y][z] == 230
                        {
                            &mut translucent_indices
                        } else {
                            &mut opaque_indices
                        };

                        block.get_model().model.draw(
                            x as f32,
                            y as f32,
                            z as f32,
                            vertices,
                            indices,
                            ViewableDirection(viewable),
                        );
                    }
                }
            }
        }

        let opaque_indices_buffer_len = opaque_indices.len() as u32;
        let translucent_indices_buffer_len = translucent_indices.len() as u32;

        // Check top faces
        UpdateChunkMesh {
            chunk: self.position,
            opaque_model: MeshData {
                vertices: opaque_vertices,
                indices: opaque_indices,
                vertices_buffer: None,
                indices_buffer: None,
                indices_buffer_len: opaque_indices_buffer_len,
            },
            translucent_model: MeshData {
                vertices: translucent_vertices,
                indices: translucent_indices,
                vertices_buffer: None,
                indices_buffer: None,
                indices_buffer_len: translucent_indices_buffer_len,
            },
            viewable_map: Some(viewable),
            model_bind_group: None,
        }
    }
}
