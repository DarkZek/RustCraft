use crate::services::chunk_service::chunk::Chunk;
use crate::services::chunk_service::mesh::culling::{ViewableDirection};
use crate::services::chunk_service::mesh::block::{draw_block};
use crate::services::chunk_service::ChunkService;
use std::collections::HashMap;
use crate::services::chunk_service::mesh::chunk::ChunkMeshData;
use crate::services::settings_service::SettingsService;
use nalgebra::{Vector3, Point3};

//
// Our greedy meshing system
//

impl Chunk {

    pub fn generate_mesh(&self, chunk_service: &ChunkService, settings: &SettingsService) -> ChunkMeshData {

        // Make sure to generate viewable map before returning null mesh data
        if self.world.is_none() {
            return ChunkMeshData {
                viewable: None,
                vertices: vec![],
                indices: vec![]
            };
        }

        // Get adjacent chunks
        let mut map = HashMap::new();
        map.insert(Vector3::new(0, 1, 0), chunk_service.chunks.get(&(self.position + Vector3::new (0, 1, 0))));
        map.insert(Vector3::new(0, -1, 0), chunk_service.chunks.get(&(self.position + Vector3::new(0, -1, 0))));
        map.insert(Vector3::new(1, 0, 0), chunk_service.chunks.get(&(self.position + Vector3::new (1, 0, 0))));
        map.insert(Vector3::new(-1, 0, 0), chunk_service.chunks.get(&(self.position + Vector3::new(-1, 0, 0))));
        map.insert(Vector3::new(0, 0, 1), chunk_service.chunks.get(&(self.position + Vector3::new (0, 0, 1))));
        map.insert(Vector3::new(0, 0, -1), chunk_service.chunks.get(&(self.position + Vector3::new(0, 0, -1))));

        let viewable = self.generate_viewable_map(map, settings.chunk_edge_faces);

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Create the buffers to add the mesh data into
        let chunk = self.world.as_ref().unwrap();

        for x in 0..chunk.len() {
            for z in 0..chunk[0][0].len() {
                for y in 0..chunk[0].len() {
                    let viewable = viewable[x][y][z].0;

                    //Isn't air
                    if chunk[x][y][z] != 0 && viewable != 0 {
                        let block = &self.blocks[chunk[x][y][z] as usize - 1];

                        //Found it, draw vertices for it
                        draw_block(Point3::new(x as f32, y as f32, z as f32), ViewableDirection(viewable), &mut vertices, &mut indices, block);
                    }
                }
            }
        }

        ChunkMeshData {
            vertices,
            indices,
            viewable: Some(viewable)
        }
    }

}