use crate::game::blocks::states::BlockStates;
use crate::services::chunk::data::{ChunkData, LightingColor, RawLightingData};
use nalgebra::Vector3;
use rc_networking::constants::CHUNK_SIZE;
use std::collections::VecDeque;

const MAX_LIGHT_VALUE: usize = 16;

pub struct LightingUpdateData {
    pub data: RawLightingData,
}

impl ChunkData {
    pub fn build_lighting(&self, states: &BlockStates) -> LightingUpdateData {
        let mut collision = [[[false; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        let mut lights = VecDeque::new();

        // Loop through chunk and set light sources
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block = states.get_block(self.world[x][y][z] as usize);
                    collision[x][y][z] = block.full;

                    if block.emission[3] == 0 {
                        continue;
                    }

                    lights.push_back((Vector3::new(x, y, z), block.emission));
                }
            }
        }

        let mut light_strengths: Vec<([u8; 4], [[[u8; 16]; 16]; 16])> = Vec::new();

        // Propagate lighting
        for (pos, color) in lights {
            assert!(color[3] <= 16);

            let mut visited = [[[false; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
            let mut strengths = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

            let mut point = VecDeque::new();

            point.push_back((pos.cast::<i32>(), color[3]));

            // Temporarily set lights to travel through the light emitting block
            let old_collision = collision[pos.x][pos.y][pos.z];
            collision[pos.x][pos.y][pos.z] = false;

            while !point.is_empty() {
                let (pos, strength) = point.pop_front().unwrap();

                if pos.x < 0
                    || pos.x == CHUNK_SIZE as i32
                    || pos.y < 0
                    || pos.y == CHUNK_SIZE as i32
                    || pos.z < 0
                    || pos.z == CHUNK_SIZE as i32
                {
                    continue;
                }

                if collision[pos.x as usize][pos.y as usize][pos.z as usize]
                    || visited[pos.x as usize][pos.y as usize][pos.z as usize]
                {
                    continue;
                }

                strengths[pos.x as usize][pos.y as usize][pos.z as usize] = strength;

                visited[pos.x as usize][pos.y as usize][pos.z as usize] = true;

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

            collision[pos.x][pos.y][pos.z] = old_collision;

            light_strengths.push((color, strengths));
        }

        // Combine strengths
        let mut data = [[[[0 as u8; 4]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let mut out_color = [0; 4];

                    // Calculate total strength and
                    let mut strength: f32 = 0.0;
                    let mut max_strength: f32 = 0.0;
                    for (_, strengths) in &light_strengths {
                        strength += strengths[x][y][z] as f32;
                        max_strength = max_strength.max(strengths[x][y][z] as f32);
                    }

                    // Add to color based on proportion of strength
                    for (color, strengths) in &light_strengths {
                        for i in 0..2 {
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
