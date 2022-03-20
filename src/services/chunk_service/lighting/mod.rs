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

        for light in lights {
            self.quality_flood_fill(light.2, light.0, light.1, &mut update);
        }

        update
    }

    // Has a tendency to use all system memory
    fn quality_flood_fill(
        &self,
        position: [usize; 3],
        color: [u8; 3],
        intensity: u8,
        update: &mut UpdateChunkLighting,
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

                // Add color to current points
                let new_color = [color[0], color[1], color[2], 255];

                apply_color_to_chunk(pos.clone(), new_color, current_intensity, update);

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
fn apply_color_to_chunk(
    mut pos: [usize; 3],
    color: [u8; 4],
    intensity: u8,
    update: &mut UpdateChunkLighting,
) {
    // Its in current chunk.
    let current_color = &mut update.chunk[pos[0]][pos[1]][pos[2]];

    let current_intensity = ((current_color[3] as f32 / 255.0) * MAX_LIGHT_LEVEL as f32) as u8;

    println!(
        "Original color: {:?} Current Intensity: {} New Color {:?} New Intensity: {}",
        current_color, current_intensity, color, intensity
    );

    if current_intensity == 0 {
        current_color[0] = color[0];
        current_color[1] = color[1];
        current_color[2] = color[2];
    } else {
        let lerp =
            ((intensity as f32 - current_intensity as f32) / (MAX_LIGHT_LEVEL as f32 * 2.0)) + 0.5;

        current_color[0] = current_color[0].lerp(color[0], lerp);
        current_color[1] = current_color[1].lerp(color[1], lerp);
        current_color[2] = current_color[2].lerp(color[2], lerp);
    }

    println!("Output {:?}", current_color);

    current_color[3] =
        current_color[3].max(((intensity as f32 / MAX_LIGHT_LEVEL as f32) * 255.0) as u8);
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
