use crate::block::blocks::BlockStates;
use crate::block::{blocks, Block};
use crate::helpers::{chunk_by_loc_from_read, chunk_by_loc_from_write};
use crate::services::chunk_service::chunk::ChunkData;
use crate::services::chunk_service::mesh::block::draw_block;
use crate::services::chunk_service::mesh::culling::ViewableDirection;
use crate::services::settings_service::SettingsService;
use nalgebra::{Point3, Vector3};
use specs::{ReadStorage, WriteStorage};
use std::collections::HashMap;

// Our mesh generation system

impl ChunkData {
    pub fn generate_mesh(&mut self, chunks: &WriteStorage<ChunkData>, settings: &SettingsService) {
        // Get adjacent chunks
        let mut map = HashMap::new();
        map.insert(
            Vector3::new(0, 1, 0),
            chunk_by_loc_from_write(chunks, (self.position + Vector3::new(0, 1, 0))),
        );
        map.insert(
            Vector3::new(0, -1, 0),
            chunk_by_loc_from_write(chunks, (self.position + Vector3::new(0, -1, 0))),
        );
        map.insert(
            Vector3::new(1, 0, 0),
            chunk_by_loc_from_write(chunks, (self.position + Vector3::new(1, 0, 0))),
        );
        map.insert(
            Vector3::new(-1, 0, 0),
            chunk_by_loc_from_write(chunks, (self.position + Vector3::new(-1, 0, 0))),
        );
        map.insert(
            Vector3::new(0, 0, 1),
            chunk_by_loc_from_write(chunks, (self.position + Vector3::new(0, 0, 1))),
        );
        map.insert(
            Vector3::new(0, 0, -1),
            chunk_by_loc_from_write(chunks, (self.position + Vector3::new(0, 0, -1))),
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
                        unsafe {
                            let block = blocks::BLOCK_STATES
                                .get()
                                .unwrap()
                                .get_block(chunk[x][y][z] as usize)
                                .unwrap();

                            let applied_color = self.light_levels[x][y][z];
                            let extra_color = self.neighboring_light_levels[x][y][z].clone();

                            let out_color = [
                                (applied_color[0] as u16 + extra_color[0] as u16) as u8,
                                (applied_color[1] as u16 + extra_color[1] as u16) as u8,
                                (applied_color[2] as u16 + extra_color[2] as u16) as u8,
                                255,
                            ];

                            let vertices = if block.transparent
                                || chunk[x][y][z] == 34
                                || chunk[x][y][z] == 230
                            {
                                &mut translucent_vertices
                            } else {
                                &mut opaque_vertices
                            };

                            // Change buffer based on transparency
                            let indices = if block.transparent {
                                &mut translucent_indices
                            } else {
                                &mut opaque_indices
                            };

                            //Found it, draw vertices for it
                            draw_block(
                                Point3::new(x as f32, y as f32, z as f32),
                                ViewableDirection(viewable),
                                vertices,
                                indices,
                                block,
                                out_color,
                            );
                        }
                    }
                }
            }
        }

        // Check top faces
        self.opaque_model.indices = opaque_indices;
        self.opaque_model.vertices = opaque_vertices;
        self.translucent_model.indices = translucent_indices;
        self.translucent_model.vertices = translucent_vertices;
        self.viewable_map = Some(viewable);
    }
}
