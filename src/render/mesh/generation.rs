use crate::render::mesh::Vertex;
use crate::world::generator::WorldGenerator;
use crate::block::Block;

pub fn generate_terrain(blocks: &Vec<Block>) -> (Vec<Vertex>, Vec<u16>) {

    let seed: f32 = rand::random();

    let world = WorldGenerator { seed: (seed * 1000.0) as u32 };
    let mut chunk = world.generate_chunk(0, 0, blocks);

    chunk.generate_mesh();

    (chunk.vertices.unwrap().clone(), chunk.indices.unwrap().clone())
}