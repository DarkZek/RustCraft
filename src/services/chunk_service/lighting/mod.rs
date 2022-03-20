use crate::block::blocks::BLOCK_STATES;
use crate::helpers::{get_chunk_coords, lerp_color, Lerp};
use crate::services::chunk_service::chunk::{ChunkData, Chunks, RawLightingData};
use crate::services::settings_service::CHUNK_SIZE;
use fnv::{FnvBuildHasher, FnvHashMap};
use nalgebra::Vector3;
use specs::{Component, DenseVecStorage};
use std::collections::HashMap;

static MAX_LIGHT_LEVEL: u8 = 16;

impl ChunkData {
    pub fn calculate_lighting(&self, chunks: &Chunks) -> UpdateChunkLighting {
        let mut lights = Vec::new();
        let states = BLOCK_STATES.get().unwrap();
        let mut update = UpdateChunkLighting::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block_id = self.world[x][y][z];

                    if block_id != 0 {
                        if let Some(block) = states.get_block(block_id as usize) {
                            if block.block_type.get_light_intensity() != 0 {
                                lights.push((
                                    block.block_type.get_light_color(),
                                    block.block_type.get_light_intensity(),
                                    [x, y, z],
                                ));
                            }
                        }
                    }
                }
            }
        }

        // The sum of all light sources affecting each square
        let mut summed_lighting = [[[([0; 4], 0); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        for light in lights {
            self.quality_flood_fill(light.2, light.0, light.1, &mut summed_lighting);
        }

        // Convert summed lighting into real lighting
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let (color, max_intensity) = summed_lighting[x][y][z];

                    if color == [0; 4] {
                        update.chunk[x][y][z] = [255, 255, 255, 0];
                        continue;
                    }

                    let color =
                        Vector3::new(color[0] as f32, color[1] as f32, color[2] as f32).normalize();

                    update.chunk[x][y][z] = [
                        (color.x * 255.0) as u8,
                        (color.y * 255.0) as u8,
                        (color.z * 255.0) as u8,
                        (max_intensity as f32 / MAX_LIGHT_LEVEL as f32 * 255.0) as u8,
                    ];
                }
            }
        }

        update
    }

    // Has a tendency to use all system memory
    fn quality_flood_fill(
        &self,
        position: [usize; 3],
        color: [u8; 3],
        intensity: u8,
        summed_lighting: &mut [[[([u64; 4], u8); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    ) {
        let mut points = Vec::with_capacity(100);
        let mut new_points = Vec::new();

        let mut current_intensity = intensity;

        let mut painted_positions = [[[false; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        points.push([
            position[0] as usize,
            position[1] as usize,
            position[2] as usize,
        ]);

        while current_intensity != 0 {
            for pos in points.iter() {
                if painted_positions[pos[0]][pos[1]][pos[2]] {
                    continue;
                }
                println!("{:?}", pos);
                let block_id = self.world[pos[0]][pos[1]][pos[2]];

                let block = BLOCK_STATES
                    .get()
                    .unwrap()
                    .get_block(block_id as usize)
                    .unwrap();

                if !block.block_type.get_transparency() && !block.block_type.is_block_full() {
                    //continue;
                }

                painted_positions[pos[0]][pos[1]][pos[2]] = true;

                apply_color_to_chunk(
                    [color[0], color[1], color[2], 255],
                    current_intensity,
                    &mut summed_lighting[pos[0]][pos[1]][pos[2]],
                );

                // Add adjacent tiles
                if pos[0] != 15 {
                    new_points.push([pos[0] + 1, pos[1], pos[2]]);
                }
                if pos[0] != 0 {
                    new_points.push([pos[0] - 1, pos[1], pos[2]]);
                }
                if pos[1] != 15 {
                    new_points.push([pos[0], pos[1] + 1, pos[2]]);
                }
                if pos[1] != 0 {
                    new_points.push([pos[0], pos[1] - 1, pos[2]]);
                }
                if pos[2] != 15 {
                    new_points.push([pos[0], pos[1], pos[2] + 1]);
                }
                if pos[2] != 0 {
                    new_points.push([pos[0], pos[1], pos[2] - 1]);
                }
            }

            points.clear();
            points = new_points.clone();
            new_points.clear();
            current_intensity -= 1;
        }
        points.clear();
    }
}

/// Applies a color to a position in a chunk.
fn apply_color_to_chunk(color: [u8; 4], intensity: u8, totals: &mut ([u64; 4], u8)) {
    // Its in current chunk.
    let (current_color, old_intensity) = totals;

    current_color[0] += color[0] as u64 * intensity as u64;
    current_color[1] += color[1] as u64 * intensity as u64;
    current_color[2] += color[2] as u64 * intensity as u64;

    // Count how many colours were added
    current_color[3] += 1;

    *old_intensity = (*old_intensity).max(intensity);
}

pub struct UpdateChunkLighting {
    pub chunk: RawLightingData,
}

impl UpdateChunkLighting {
    pub fn new() -> UpdateChunkLighting {
        UpdateChunkLighting {
            chunk: [[[[255, 255, 255, 0]; 16]; 16]; 16],
        }
    }
}

impl Component for UpdateChunkLighting {
    type Storage = DenseVecStorage<Self>;
}
