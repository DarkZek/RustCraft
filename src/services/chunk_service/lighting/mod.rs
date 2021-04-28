use crate::block::blocks::{BlockStates, BLOCK_STATES};
use crate::helpers::lerp_color;
use crate::services::chunk_service::chunk::{ChunkData, Chunks, RawLightingData};
use crate::services::settings_service::CHUNK_SIZE;
use nalgebra::{Point3, Vector3};
use specs::{Component, DenseVecStorage, ReadStorage};
use std::collections::HashMap;

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
            self.quality_flood_fill(light.2, light.0, light.1, chunks, &mut update);
        }

        update
    }

    // Has a tendency to use all system memory
    fn quality_flood_fill(
        &self,
        position: [usize; 3],
        color: [u8; 3],
        intensity: u8,
        chunks: &Chunks,
        update: &mut UpdateChunkLighting,
    ) {
        let mut points = Vec::new();
        let mut new_points = Vec::new();

        let mut current_intensity = intensity;

        let mut painted_positions = Vec::new();

        points.push([position[0] as i32, position[1] as i32, position[2] as i32]);

        while current_intensity != 0 {
            for pos in points.iter() {
                if painted_positions.contains(pos) {
                    continue;
                }

                painted_positions.push(*pos);

                // Add color to current points
                let new_color = [color[0], color[1], color[2], 255];
                apply_color_to_chunk(self, pos.clone(), new_color, intensity, chunks, update);

                // Add adjacent tiles
                new_points.push([pos[0] + 1, pos[1], pos[2]]);
                new_points.push([pos[0] - 1, pos[1], pos[2]]);
                new_points.push([pos[0], pos[1] + 1, pos[2]]);
                new_points.push([pos[0], pos[1] - 1, pos[2]]);
                new_points.push([pos[0], pos[1], pos[2] + 1]);
                new_points.push([pos[0], pos[1], pos[2] - 1]);
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
/// Sometimes this position can be greater than the size of a single chunk so it spreads into a maximum of one chunk away in all directions.
fn apply_color_to_chunk(
    chunk: &ChunkData,
    mut pos: [i32; 3],
    color: [u8; 4],
    intensity: u8,
    chunks: &Chunks,
    update: &mut UpdateChunkLighting,
) {
    if pos[0] >= 0 && pos[0] <= 15 && pos[1] >= 0 && pos[1] <= 15 && pos[2] >= 0 && pos[2] <= 15 {
        // Its in current chunk.
        let current_color = &mut update.chunk[pos[0] as usize][pos[1] as usize][pos[2] as usize];
        let intensity_to_rgb = 25.0;

        let new_intensity = (current_color[3] as f32)
            .max(intensity as f32 * intensity_to_rgb)
            .max(intensity_to_rgb) as u8;
        *current_color = if current_color == &[0, 0, 0, 50] {
            color
        } else {
            lerp_color(current_color.clone(), color, 0.5)
        };
        current_color[3] = new_intensity;
    } else {
        // Its in other chunk
        let mut chunk_pos = chunk.position;

        // Calculate chunk position
        if pos[0] < 0 {
            chunk_pos.x -= 1;
            pos[0] += CHUNK_SIZE as i32;
        } else if pos[0] > 15 {
            chunk_pos.x += 1;
            pos[0] -= CHUNK_SIZE as i32;
        }

        if pos[1] < 0 {
            chunk_pos.y -= 1;
            pos[1] += CHUNK_SIZE as i32;
        } else if pos[1] > 15 {
            chunk_pos.y += 1;
            pos[1] -= CHUNK_SIZE as i32;
        }

        if pos[2] < 0 {
            chunk_pos.z -= 1;
            pos[2] += CHUNK_SIZE as i32;
        } else if pos[2] > 15 {
            chunk_pos.z += 1;
            pos[2] -= CHUNK_SIZE as i32;
        }

        // Make sure chunk exists
        let adjacent_chunk = if update.adjacent.contains_key(&chunk_pos) {
            update.adjacent.get_mut(&chunk_pos).unwrap()
        } else {
            update
                .adjacent
                .insert(chunk_pos, [[[[0, 0, 0, 0]; 16]; 16]; 16]);
            update.adjacent.get_mut(&chunk_pos).unwrap()
        };

        let current_color = &mut adjacent_chunk[pos[0] as usize][pos[1] as usize][pos[2] as usize];

        let intensity_to_rgb = 20.0;

        let new_intensity = (current_color[3] as f32)
            .max(intensity as f32 * intensity_to_rgb)
            .max(intensity_to_rgb) as u8;
        *current_color = if current_color == &[0, 0, 0, 50] {
            color
        } else {
            lerp_color(current_color.clone(), color, 0.5)
        };
        current_color[3] = new_intensity;
    }
}

pub struct UpdateChunkLighting {
    pub chunk: RawLightingData,
    pub adjacent: HashMap<Vector3<i32>, RawLightingData>,
}

impl UpdateChunkLighting {
    pub fn new() -> UpdateChunkLighting {
        UpdateChunkLighting {
            chunk: [[[[0, 0, 0, 50]; 16]; 16]; 16],
            adjacent: HashMap::new(),
        }
    }
}

impl Component for UpdateChunkLighting {
    type Storage = DenseVecStorage<Self>;
}
