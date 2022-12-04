use crate::game::blocks::states::BlockStates;
use crate::game::blocks::Block;
use crate::helpers::{
    check_chunk_boundaries, get_chunk_coords, global_to_local_position, MAX_LIGHT_VALUE,
};
use crate::systems::chunk::data::{ChunkData, LightingColor, RawLightingData};
use crate::systems::chunk::nearby_cache::NearbyChunkCache;
use bevy::prelude::system_adapter::new;
use bevy::prelude::Entity;
use nalgebra::{max, Vector3, Vector4};
use rc_networking::constants::CHUNK_SIZE;
use std::collections::VecDeque;
use std::mem;
use std::ops::Range;
use std::time::Instant;

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

        let mut lights = self.get_lights(states, cache);

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
            // a map of where the light has visited
            let mut visited = [[[false; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

            let mut point = VecDeque::with_capacity(100);

            // Starting points
            point.push_back((light_pos, color[3]));

            while !point.is_empty() {
                let (pos, strength) = point.pop_front().unwrap();

                if visited[pos.x][pos.y][pos.z] {
                    continue;
                }

                if !states
                    .get_block(self.world[pos.x][pos.y][pos.z] as usize)
                    .translucent
                {
                    // Collision, bail
                    continue;
                }

                let current_color = &mut data[pos.x][pos.y][pos.z];

                current_color[0] += strength as u32 * color[0] as u32;
                current_color[1] += strength as u32 * color[1] as u32;
                current_color[2] += strength as u32 * color[2] as u32;
                current_color[3] += strength as u32;
                current_color[4] = current_color[4].max(strength as u32);

                visited[pos.x][pos.y][pos.z] = true;

                if strength == 1 {
                    continue;
                }

                for side in &BLOCK_SIDES {
                    if !check_chunk_boundaries(pos, *side) {
                        continue;
                    }

                    let new_pos = Vector3::new(
                        (pos.x as i32 + side.x) as usize,
                        (pos.y as i32 + side.y) as usize,
                        (pos.z as i32 + side.z) as usize,
                    );

                    if visited[new_pos.x][new_pos.y][new_pos.z] {
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

                    out_color[3] = color[4] as u8;
                }
            }
        }

        println!(
            "Took {}ns to render {:?} with {} lights with flood fill",
            start.elapsed().as_nanos(),
            self.position,
            lights_len
        );

        LightingUpdateData { data: out }
    }

    pub fn build_lighting_blur(
        &self,
        states: &BlockStates,
        cache: &NearbyChunkCache,
    ) -> LightingUpdateData {
        let start = Instant::now();

        let mut out = [[[[0 as u8; 4]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        let mut collision = [[[false; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block = states.get_block(self.world[x][y][z] as usize);

                    collision[x][y][z] = !block.translucent;

                    if block.emission[3] == 0 {
                        continue;
                    }

                    println!("Found emitter at {} {} {}", x, y, z);

                    out[x][y][z] = block.emission;
                    collision[x][y][z] = true;
                }
            }
        }

        let mut changed = true;
        while (changed) {
            changed = false;
            //println!("pass");
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        if collision[x][y][z] {
                            continue;
                        }

                        // Average 6 surrounding blocks
                        let mut avg = Vector4::<u32>::zeros();
                        let mut max_strength: u8 = 0;
                        for side in &BLOCK_SIDES {
                            let (chunk_pos, block_pos) = global_to_local_position(
                                side + Vector3::new(x, y, z).cast::<i32>() + (self.position * 16),
                            );

                            // If its this chunk then read this chunks lighting data
                            if chunk_pos == self.position {
                                avg += Vector4::new(
                                    out[block_pos.x][block_pos.y][block_pos.z][0] as u32
                                        * out[block_pos.x][block_pos.y][block_pos.z][3] as u32,
                                    out[block_pos.x][block_pos.y][block_pos.z][1] as u32
                                        * out[block_pos.x][block_pos.y][block_pos.z][3] as u32,
                                    out[block_pos.x][block_pos.y][block_pos.z][2] as u32
                                        * out[block_pos.x][block_pos.y][block_pos.z][3] as u32,
                                    out[block_pos.x][block_pos.y][block_pos.z][3] as u32,
                                );
                                max_strength =
                                    max_strength.max(out[block_pos.x][block_pos.y][block_pos.z][3]);
                                //println!("{:?} {} X {} {} {}", avg, max_strength, x, y, z);
                                continue;
                            }

                            if let Some(v) = cache.get_chunk(chunk_pos) {
                                avg += Vector4::new(
                                    v.light_levels[block_pos.x][block_pos.y][block_pos.z][0] as u32
                                        * out[block_pos.x][block_pos.y][block_pos.z][3] as u32,
                                    v.light_levels[block_pos.x][block_pos.y][block_pos.z][1] as u32
                                        * out[block_pos.x][block_pos.y][block_pos.z][3] as u32,
                                    v.light_levels[block_pos.x][block_pos.y][block_pos.z][2] as u32
                                        * out[block_pos.x][block_pos.y][block_pos.z][3] as u32,
                                    v.light_levels[block_pos.x][block_pos.y][block_pos.z][3] as u32,
                                );
                                max_strength = max_strength
                                    .max(v.light_levels[block_pos.x][block_pos.y][block_pos.z][3]);
                            }
                        }

                        if max_strength == 0 || avg.w == 0 {
                            continue;
                        }

                        //println!("{:?} {}", avg, max_strength);

                        avg /= avg.w;
                        max_strength -= 1;

                        //println!("bb {:?} {}", avg, max_strength);

                        if out[x][y][z]
                            != [avg.x as u8, avg.y as u8, avg.z as u8, max_strength.min(16)]
                        {
                            //println!("modify {:?}", out[x][y][z]);
                            out[x][y][z] =
                                [avg.x as u8, avg.y as u8, avg.z as u8, max_strength.min(16)];

                            //println!("chabgwed");
                            changed = true;
                        }
                    }
                }
            }

            //println!("{:?}", out);
        }

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let out_color = &mut out[x][y][z];

                    // If there's no lighting data for this block ignore it
                    if out_color[3] == 0 {
                        continue;
                    }

                    out_color[3] = out_color[3] as u8;
                }
            }
        }

        println!(
            "Took {}ns to render {:?} with with blur",
            start.elapsed().as_nanos(),
            self.position,
        );

        LightingUpdateData { data: out }
    }

    // Gets all the lights in this chunk and the surrounding chunks
    fn get_lights(
        &self,
        states: &BlockStates,
        cache: &NearbyChunkCache,
    ) -> Vec<(Vector3<usize>, [u8; 4])> {
        let mut lights = Vec::new();

        // Direct lights in this scene
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block = states.get_block(self.world[x][y][z] as usize);

                    if block.emission[3] == 0 {
                        continue;
                    }

                    lights.push((Vector3::new(x, y, z), block.emission));
                }
            }
        }

        // Light from above
        self.collect_neighbor_lights(
            0..(CHUNK_SIZE as i32),
            16..17,
            0..(CHUNK_SIZE as i32),
            Vector3::new(0, -1, 0),
            cache,
            &mut lights,
        );

        // Light from below
        self.collect_neighbor_lights(
            0..(CHUNK_SIZE as i32),
            -1..0,
            0..(CHUNK_SIZE as i32),
            Vector3::new(0, 1, 0),
            cache,
            &mut lights,
        );

        // Light from the front
        self.collect_neighbor_lights(
            0..(CHUNK_SIZE as i32),
            0..(CHUNK_SIZE as i32),
            16..17,
            Vector3::new(0, 0, -1),
            cache,
            &mut lights,
        );

        // Light from the back
        self.collect_neighbor_lights(
            0..(CHUNK_SIZE as i32),
            0..(CHUNK_SIZE as i32),
            -1..0,
            Vector3::new(0, 0, 1),
            cache,
            &mut lights,
        );

        // Light from the left
        self.collect_neighbor_lights(
            16..17,
            0..(CHUNK_SIZE as i32),
            0..(CHUNK_SIZE as i32),
            Vector3::new(-1, 0, 0),
            cache,
            &mut lights,
        );

        // Light from the right
        self.collect_neighbor_lights(
            -1..0,
            0..(CHUNK_SIZE as i32),
            0..(CHUNK_SIZE as i32),
            Vector3::new(1, 0, 0),
            cache,
            &mut lights,
        );

        lights
    }

    fn collect_neighbor_lights(
        &self,
        x_range: Range<i32>,
        y_range: Range<i32>,
        z_range: Range<i32>,
        offset: Vector3<i32>,
        cache: &NearbyChunkCache,
        lights: &mut Vec<(Vector3<usize>, [u8; 4])>,
    ) {
        // Check lighting coming from an above chunk
        for x in x_range {
            for y in y_range.clone() {
                for z in z_range.clone() {
                    let (chunk_pos, local_pos) = global_to_local_position(
                        Vector3::new(x, y, z) + (self.position * CHUNK_SIZE as i32),
                    );

                    if let Some(chunk) = cache.get_chunk(chunk_pos) {
                        let light = chunk.light_levels[local_pos.x][local_pos.y][local_pos.z];
                        if light[3] > 1 {
                            // Add light
                            lights.push((
                                Vector3::new(
                                    (x + offset.x) as usize,
                                    (y + offset.y) as usize,
                                    (z + offset.z) as usize,
                                ),
                                [light[0], light[1], light[2], light[3] - 1],
                            ))
                        }
                    }
                }
            }
        }
    }
}

const BLOCK_SIDES: [Vector3<i32>; 6] = [
    Vector3::new(1, 0, 0),
    Vector3::new(-1, 0, 0),
    Vector3::new(0, 1, 0),
    Vector3::new(0, -1, 0),
    Vector3::new(0, 0, 1),
    Vector3::new(0, 0, -1),
];
