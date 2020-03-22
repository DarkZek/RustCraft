use crate::world::chunk::Chunk;
use crate::render::mesh::Vertex;
use crate::render::mesh::culling::{calculate_viewable, ViewableDirection};
use crate::render::mesh::block::draw_block;

impl Chunk {
    pub fn generate_mesh(&mut self) {
        let mut vertices: Vec<Vertex> = Vec::with_capacity(100_000);
        let mut indices: Vec<u16> = Vec::with_capacity(100_000);
        let world = self.world;

        for x in 0..world.len() {
            for z in 0..world[0][0].len() {
                for y in 0..world[0].len() {
                    let viewable = calculate_viewable(&self, [x, y, z]);

                    //Isn't air
                    if world[x][y][z] != 0 {
                        let block = &self.blocks[world[x][y][z] as usize - 1];

                        //Found it, draw vertices for it
                        draw_block(x as f32, y as f32, z as f32, ViewableDirection(viewable), &mut vertices, &mut indices, block);
                    }
                }
            }
        }

        self.vertices = Some(vertices);
        self.indices = Some(indices);
    }
}