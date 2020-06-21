use noise::{NoiseFn, Perlin, Seedable};
use crate::block::Block;
use crate::services::settings_service::{CHUNK_SIZE};
use crate::services::chunk_service::chunk::{ChunkData};
use nalgebra::Vector3;

pub struct World {

}

impl World {

    pub fn generate_chunk(chunk_pos: Vector3<i32>, blocks: &Vec<Block>) -> Option<ChunkData> {
        let scale = 1.0 / CHUNK_SIZE as f64;
        let mut chunk_nothing = true;

        let noise_map = Perlin::new();
        noise_map.set_seed(0);

        let mut chunk = [[[0 as u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        let blocks: Vec<Block> = (*blocks).to_vec();

        for x in 0..chunk.len() {
            for z in 0..chunk[0][0].len() {
                for y_offset in 0..CHUNK_SIZE {
                    let y = (chunk_pos.y * CHUNK_SIZE as i32) + y_offset as i32;
                    let height_map = noise_map.get([(x as f64 * scale) + chunk_pos.x as f64, (z as f64 * scale) + chunk_pos.z as f64]);
                    let height = (height_map * 20.0).round() as i32 + 50;

                    // Stone
                    if y < height {
                        chunk[x][y_offset][z] = 1;
                        chunk_nothing = false;
                    // Dirt
                    } else if y <= (height + 1) {
                        chunk[x][y_offset][z] = 2;
                        chunk_nothing = false;
                    } else if y == (height + 2) {
                        chunk[x][y_offset][z] = 3;
                        chunk_nothing = false;
                    }

                    // chunk[x][y_offset][z] = 0;
                    // if ((x * 5) + (y_offset * 3) + (z * 2)) % 20 == 0 {
                    //     chunk[x][y_offset][z] = 1;
                    // }
                    //
                    // if chunk_pos.x == 0 && chunk_pos.z == 0 {
                    //     chunk[x][y_offset][z] = 2;
                    // }
                    // if chunk_pos.x == 1 && chunk_pos.z == 0 {
                    //     chunk[x][y_offset][z] = 3;
                    // }
                    // if chunk_pos.y == 15 {
                    //     chunk[x][y_offset][z] = 0;
                    // }
                }
            }
        }

        // Way to tell X/Z
        // chunk[0][0][0] = 1;
        // chunk[1][0][0] = 1;
        // chunk[2][0][0] = 1;
        // chunk[3][0][0] = 1;
        //
        // chunk[0][0][1] = 2;
        // chunk[0][0][1] = 2;
        // chunk[0][0][1] = 2;
        // chunk[0][0][1] = 2;
        if chunk_nothing {
            None
        } else {
            Some((chunk, blocks))
        }
    }
}