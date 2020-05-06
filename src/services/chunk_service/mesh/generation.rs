use crate::services::chunk_service::chunk::Chunk;
use crate::services::chunk_service::mesh::culling::{ViewableDirection};
use crate::services::chunk_service::mesh::block::{draw_block};
use cgmath::{Point3, Vector3};
use crate::services::chunk_service::ChunkService;
use std::collections::HashMap;
use crate::services::chunk_service::mesh::chunk::ChunkMeshData;

//
// Our greedy meshing system
//

impl Chunk {

    pub fn generate_mesh(&self, chunk_service: &ChunkService) -> ChunkMeshData {

        // Get adjacent chunks
        let mut map = HashMap::new();
        map.insert(Vector3 { x: 0, y: 1, z: 0 }, chunk_service.chunks.get(&(self.position + Vector3{x: 0, y: 1, z: 0})));
        map.insert(Vector3 { x: 0, y: -1, z: 0 }, chunk_service.chunks.get(&(self.position + Vector3{x: 0, y: -1, z: 0})));
        map.insert(Vector3 { x: 1, y: 0, z: 0 }, chunk_service.chunks.get(&(self.position + Vector3{x: 1, y: 0, z: 0})));
        map.insert(Vector3 { x: -1, y: 0, z: 0 }, chunk_service.chunks.get(&(self.position + Vector3{x: -1, y: 0, z: 0})));
        map.insert(Vector3 { x: 0, y: 0, z: 1 }, chunk_service.chunks.get(&(self.position + Vector3{x: 0, y: 1, z: 0})));
        map.insert(Vector3 { x: 0, y: 0, z: -1 }, chunk_service.chunks.get(&(self.position + Vector3{x: 0, y: -1, z: 0})));

        let viewable = self.generate_viewable_map(map);

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Create the buffers to add the mesh data into
        let chunk = self.world;

        for x in 0..chunk.len() {
            for z in 0..chunk[0][0].len() {
                for y in 0..chunk[0].len() {
                    let viewable = viewable[x][y][z].0;

                    //Isn't air
                    if chunk[x][y][z] != 0 && viewable != 0 {
                        let block = &self.blocks[chunk[x][y][z] as usize - 1];

                        //Found it, draw vertices for it
                        draw_block(Point3 {
                            x: x as f32,
                            y: y as f32,
                            z: z as f32
                        }, ViewableDirection(viewable), &mut vertices, &mut indices, block);
                    }
                }
            }
        }

        ChunkMeshData {
            vertices,
            indices,
            viewable
        }
    }

}