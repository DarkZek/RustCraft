use crate::game::blocks::states::BlockStates;
use crate::game::blocks::Block;
use crate::helpers::{get_chunk_coords, global_to_local_position};
use crate::services::chunk::data::{ChunkData, LightingColor, RawLightingData};
use crate::services::chunk::nearby_cache::NearbyChunkCache;
use nalgebra::Vector3;
use rc_networking::constants::CHUNK_SIZE;
use std::collections::VecDeque;
use std::time::Instant;

const MAX_LIGHT_VALUE: usize = 16;

pub struct LightingUpdateData {
    pub data: RawLightingData,
}

impl ChunkData {
    pub fn build_lighting(
        &self,
        states: &BlockStates,
        cache: &NearbyChunkCache,
    ) -> LightingUpdateData {
        let mut lights = get_lights(self.position, states, cache);

        // Loop through each light and calculate its individual impact
        let mut light_strengths: Vec<([u8; 4], [[[u8; 16]; 16]; 16])> = Vec::new();

        // Propagate lighting
        for (light_pos, color) in lights {
            assert!(color[3] <= 16);

            // a 32 block wide area around the light that tracks what blocks its visited
            let mut visited = [[[false; CHUNK_SIZE * 2]; CHUNK_SIZE * 2]; CHUNK_SIZE * 2];
            let mut strengths = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

            let mut point = VecDeque::with_capacity(100);

            // Starting points
            point.push_back((light_pos + Vector3::new(1, 0, 0), color[3] - 1));
            point.push_back((light_pos - Vector3::new(1, 0, 0), color[3] - 1));
            point.push_back((light_pos + Vector3::new(0, 1, 0), color[3] - 1));
            point.push_back((light_pos - Vector3::new(0, 1, 0), color[3] - 1));
            point.push_back((light_pos + Vector3::new(0, 0, 1), color[3] - 1));
            point.push_back((light_pos - Vector3::new(0, 0, 1), color[3] - 1));

            while !point.is_empty() {
                let (pos, strength) = point.pop_front().unwrap();

                let (chunk_pos, block_pos) = global_to_local_position(pos);

                if visited[(pos.x - light_pos.x + CHUNK_SIZE as i32) as usize]
                    [(pos.y - light_pos.y + CHUNK_SIZE as i32) as usize]
                    [(pos.z - light_pos.z + CHUNK_SIZE as i32) as usize]
                {
                    continue;
                }

                if let Some(v) = cache.get_chunk(chunk_pos) {
                    assert_eq!(v.position, chunk_pos);

                    if !states
                        .get_block(v.world[block_pos.x][block_pos.y][block_pos.z] as usize)
                        .translucent
                    {
                        // Collision, bail
                        continue;
                    }
                }

                if chunk_pos == self.position {
                    strengths[block_pos.x as usize][block_pos.y as usize][block_pos.z as usize] =
                        strength;
                }

                visited[(pos.x - light_pos.x + CHUNK_SIZE as i32) as usize]
                    [(pos.y - light_pos.y + CHUNK_SIZE as i32) as usize]
                    [(pos.z - light_pos.z + CHUNK_SIZE as i32) as usize] = true;

                if strength == 1 {
                    continue;
                }

                point.push_back((pos + Vector3::new(1, 0, 0), strength - 1));
                point.push_back((pos - Vector3::new(1, 0, 0), strength - 1));
                point.push_back((pos + Vector3::new(0, 1, 0), strength - 1));
                point.push_back((pos - Vector3::new(0, 1, 0), strength - 1));
                point.push_back((pos + Vector3::new(0, 0, 1), strength - 1));
                point.push_back((pos - Vector3::new(0, 0, 1), strength - 1));
            }

            light_strengths.push((color, strengths));
        }

        // Combine strengths
        let mut data = [[[[0 as u8; 4]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let mut out_color = [0; 4];

                    // Calculate total strength
                    let mut strength: f32 = 0.0;
                    let mut max_strength: f32 = 0.0;
                    for (_, strengths) in &light_strengths {
                        strength += strengths[x][y][z] as f32;
                        max_strength = max_strength.max(strengths[x][y][z] as f32);
                    }

                    // Add to color based on proportion of strength
                    for (color, strengths) in &light_strengths {
                        for i in 0..=2 {
                            out_color[i] += (color[i] as f32
                                * (strengths[x][y][z] as f32 / strength)
                                * (max_strength / MAX_LIGHT_VALUE as f32))
                                .floor() as u8
                        }
                    }

                    out_color[3] = 255;

                    data[x][y][z] = out_color;
                }
            }
        }

        LightingUpdateData { data }
    }
}

// Gets all the lights in this chunk and the surrounding chunks
fn get_lights(
    chunk_pos: Vector3<i32>,
    states: &BlockStates,
    cache: &NearbyChunkCache,
) -> Vec<(Vector3<i32>, [u8; 4])> {
    let mut lights = Vec::new();

    for chunk_x in (chunk_pos.x - 1)..=(chunk_pos.x + 1) {
        for chunk_y in (chunk_pos.y - 1)..=(chunk_pos.y + 1) {
            for chunk_z in (chunk_pos.z - 1)..=(chunk_pos.z + 1) {
                // Get chunk
                if let Some(chunk) = cache.get_chunk(Vector3::new(chunk_x, chunk_y, chunk_z)) {
                    for x in 0..CHUNK_SIZE {
                        for y in 0..CHUNK_SIZE {
                            for z in 0..CHUNK_SIZE {
                                let block = states.get_block(chunk.world[x][y][z] as usize);

                                if block.emission[3] == 0 {
                                    continue;
                                }

                                lights.push((
                                    Vector3::new(x, y, z).cast::<i32>()
                                        + (CHUNK_SIZE as i32 * chunk.position),
                                    block.emission,
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    lights
}
