use crate::block::blocks::BlockStates;
use crate::helpers::lerp_color;
use crate::services::chunk_service::chunk::ChunkData;
use crate::services::settings_service::CHUNK_SIZE;
use nalgebra::Point3;
use specs::{Join, ReadStorage, WriteStorage};

impl ChunkData {
    pub fn calculate_lighting(
        &mut self,
        chunks: &mut WriteStorage<ChunkData>,
        blocks: BlockStates,
    ) {
        let mut lights = Vec::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block_id = self.world[x][y][z];

                    if block_id != 0 {
                        let block = blocks.get_block(block_id as usize).unwrap();

                        lights.push((
                            block.block_type.get_light_color(),
                            block.block_type.get_light_intensity(),
                            [x, y, z],
                        ));
                    }
                }
            }
        }

        for light in lights {
            self.quality_flood_fill(light.2, light.0, light.1, chunks);
        }
    }

    fn fast_flood_fill(
        &mut self,
        position: [usize; 3],
        _color: [f32; 3],
        intensity: u8,
        _chunks: &mut ReadStorage<ChunkData>,
    ) {
        let posx = position[0] as i8;
        let posy = position[1] as i8;
        let posz = position[2] as i8;

        for x in position[0] as i8 - intensity as i8..posx + intensity as i8 {
            if x < 0 || x > CHUNK_SIZE as i8 - 1 {
                continue;
            }
            for y in (posy - intensity as i8)..(posy + intensity as i8) {
                if y < 0 || y > CHUNK_SIZE as i8 - 1 {
                    continue;
                }
                for z in (posz - intensity as i8)..(posz + intensity as i8) {
                    if z < 0 || z > CHUNK_SIZE as i8 - 1 {
                        continue;
                    }
                    let light;
                    let distance = crate::helpers::distance(
                        &Point3::new(position[0], position[1], position[2]),
                        &Point3::new(x as usize, y as usize, z as usize),
                    );
                    if distance > intensity as u32 {
                        light = 0.0;
                    } else {
                        light = 1.0 / distance as f32;
                    }
                    // self.light_levels[x as usize][y as usize][z as usize] =
                    //     [light, color[0], color[1], color[2]];
                }
            }
        }
    }

    // Has a tendency to use all system memory
    fn quality_flood_fill(
        &mut self,
        position: [usize; 3],
        color: [u8; 3],
        intensity: u8,
        chunks: &mut WriteStorage<ChunkData>,
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

                let col = current_intensity as f32 / 32.0;
                //let col = 1.0;

                // Add color to current points
                let new_color = [color[0], color[1], color[2], 255];
                apply_color_to_chunk(self, pos.clone(), new_color, col, chunks);

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
    chunk: &mut ChunkData,
    mut pos: [i32; 3],
    color: [u8; 4],
    intensity: f32,
    chunks: &mut WriteStorage<ChunkData>,
) {
    if pos[0] >= 0 && pos[0] <= 15 && pos[1] >= 0 && pos[1] <= 15 && pos[2] >= 0 && pos[2] <= 15 {
        // Its in current chunk.
        let current_color =
            &mut chunk.light_levels[pos[0] as usize][pos[1] as usize][pos[2] as usize];
        *current_color = lerp_color(color, *current_color, intensity);
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

        let chunk = {
            let mut c = None;
            for mut chunk in chunks.join() {
                if chunk.position.eq(&chunk_pos) {
                    c = Some(chunk);
                }
            }
            c
        };

        // Make sure chunk exists
        if let Some(chunk) = chunk {
            let current_color = &mut chunk.neighboring_light_levels[pos[0] as usize]
                [pos[1] as usize][pos[2] as usize];

            let new_intensity = current_color[3] + (intensity * 25.0) as u8;
            *current_color = lerp_color(current_color.clone(), color, intensity);
            current_color[3] = new_intensity;
        }
    }
}
