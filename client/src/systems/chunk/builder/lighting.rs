use crate::systems::chunk::data::ChunkData;
use crate::systems::chunk::nearby_cache::NearbyChunkCache;
use bevy::log::{debug};
use nalgebra::Vector3;
use rc_shared::block::BlockStates;
use rc_shared::chunk::RawLightingData;
use rc_shared::helpers::global_to_local_position;
use rc_shared::viewable_direction::BLOCK_SIDES;
use rc_shared::CHUNK_SIZE;
use std::collections::VecDeque;
use web_time::Instant;

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
        let start = Instant::now();

        let lights = get_lights(self.position, states, cache);

        if lights.len() == 0 {
            return LightingUpdateData {
                data: [[[[0 as u8; 4]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            };
        }

        let lights_len = lights.len();

        // Rolling average of chunk lighting data
        let mut data = [[[[0 as u32; 5]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        // Propagate lighting
        for (light_pos, color) in lights {
            // a 32 block wide area around the light that tracks what blocks its visited
            let mut visited = [[[false; CHUNK_SIZE * 2]; CHUNK_SIZE * 2]; CHUNK_SIZE * 2];

            let mut point = VecDeque::with_capacity(1000);

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
                    let current_color =
                        &mut data[block_pos.x as usize][block_pos.y as usize][block_pos.z as usize];
                    current_color[0] += strength as u32 * color[0] as u32;
                    current_color[1] += strength as u32 * color[1] as u32;
                    current_color[2] += strength as u32 * color[2] as u32;
                    current_color[3] += strength as u32;
                    current_color[4] = current_color[4].max(strength as u32);
                }

                visited[(pos.x - light_pos.x + CHUNK_SIZE as i32) as usize]
                    [(pos.y - light_pos.y + CHUNK_SIZE as i32) as usize]
                    [(pos.z - light_pos.z + CHUNK_SIZE as i32) as usize] = true;

                if strength == 1 {
                    continue;
                }

                for side in &BLOCK_SIDES {
                    let new_pos = pos + side;

                    if visited[(new_pos.x - light_pos.x + CHUNK_SIZE as i32) as usize]
                        [(new_pos.y - light_pos.y + CHUNK_SIZE as i32) as usize]
                        [(new_pos.z - light_pos.z + CHUNK_SIZE as i32) as usize]
                    {
                        continue;
                    }

                    point.push_back((new_pos, strength - 1));
                }
            }
        }

        // Combine strengths
        let mut out = [[[[0 as u8; 4]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let color = &data[x][y][z];

                    // If there's no lighting data for this block ignore it
                    if color[3] == 0 {
                        continue;
                    }

                    let out_color = &mut out[x][y][z];

                    // Get the proportional color
                    out_color[0] = (color[0] / color[3]) as u8;
                    out_color[1] = (color[1] / color[3]) as u8;
                    out_color[2] = (color[2] / color[3]) as u8;

                    // Light falloff
                    out_color[0] =
                        (out_color[0] as u32 * color[4] as u32 / MAX_LIGHT_VALUE as u32) as u8;
                    out_color[1] =
                        (out_color[1] as u32 * color[4] as u32 / MAX_LIGHT_VALUE as u32) as u8;
                    out_color[2] =
                        (out_color[2] as u32 * color[4] as u32 / MAX_LIGHT_VALUE as u32) as u8;

                    out_color[3] = color[4] as u8;
                }
            }
        }

        debug!(
            "Took {}ns to render {:?} with {} lights with flood fill",
            start.elapsed().as_nanos(),
            self.position,
            lights_len
        );

        LightingUpdateData { data: out }
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