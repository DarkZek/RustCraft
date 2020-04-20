use noise::{NoiseFn, Perlin, Seedable};
use crate::block::Block;
use crate::services::settings_service::{CHUNK_SIZE, CHUNK_HEIGHT};
use crate::services::chunk_service::chunk::{ChunkData};

pub struct World {
}

impl World {

    pub fn generate_chunk(chunk_x: i32, chunk_z: i32, blocks: &Vec<Block>) -> ChunkData {
        let scale = 1.0 / CHUNK_SIZE as f64;

        let noise_map = Perlin::new();
        noise_map.set_seed(0);

        let mut world = [[[0 as u32; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE];
        let blocks: Vec<Block> = (*blocks).to_vec();

        for x in 0..world.len() {
            for z in 0..world[0][0].len() {
                let height_map = noise_map.get([(x as f64 * scale) + chunk_x as f64, (z as f64 * scale) + chunk_z as f64]);
                let height = (height_map * 20.0).round() as i32;

                for y in 0..(height + 50) {
                    world[x][y as usize][z] = 1;
                }

                //Dirt & grass
                world[x][(height + 50) as usize][z] = 2;
                world[x][(height + 51) as usize][z] = 2;
                world[x][(height + 52) as usize][z] = 3;
            }
        }

        (world, blocks)
    }

    pub fn generate_flat_chunk(blocks: &Vec<Block>) -> ChunkData {
        let mut world = [[[0 as u32; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE];
        let blocks: Vec<Block> = blocks.clone();

        for x in 0..world.len() {
            for z in 0..world[0][0].len() {

                for y in 0..40 {
                    world[x][y as usize][z] = 1;
                }

                //Dirt & grass
                world[x][(40) as usize][z] = 2;
                world[x][(41) as usize][z] = 2;
                world[x][(42) as usize][z] = 3;
            }
        }

        (world, blocks)
    }
}