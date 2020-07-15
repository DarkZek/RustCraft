use crate::services::chunk_service::chunk::{ChunkData, Chunk};
use crate::services::settings_service::CHUNK_SIZE;
use crate::block::Block;
use nalgebra::{Point3};
use crate::services::chunk_service::ChunkService;
use crate::helpers::lerp_color;

impl ChunkData {
    pub fn calculate_lighting(&mut self, service: &mut ChunkService) {

        let mut lights = Vec::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block_id = self.world[x][y][z];

                    if block_id != 0 {
                        let block: &Block = self.blocks.get(block_id as usize - 1).unwrap();

                        lights.push((block.color, block.light_intensity, [x, y, z]));
                    }
                }
            }
        }

        for light in lights {
            self.quality_flood_fill(light.2, light.0, light.1, service);
        }
    }

    fn fast_flood_fill(&mut self, position: [usize; 3], color: [f32; 3], intensity: u8, service: &mut ChunkService) {
        let posx = position[0] as i8;
        let posy = position[1] as i8;
        let posz = position[2] as i8;

        for x in position[0] as i8 - intensity as i8 .. posx + intensity as i8 {
            if x < 0 || x > CHUNK_SIZE as i8 - 1 { continue; }
            for y in (posy - intensity as i8) .. (posy + intensity as i8) {
                if y < 0 || y > CHUNK_SIZE as i8 - 1 { continue; }
                for z in (posz - intensity as i8) .. (posz + intensity as i8) {
                    if z < 0 || z > CHUNK_SIZE as i8 - 1 { continue; }
                    let light;
                    let distance = crate::helpers::distance(&Point3::new(position[0], position[1], position[2]), &Point3::new(x as usize, y as usize, z as usize));
                    if distance > intensity as u32 {
                        light = 0.0;
                    } else {
                        light = 1.0 / distance as f32;
                    }
                    self.light_levels[x as usize][y as usize][z as usize] = [light, color[0], color[1], color[2]];
                }
            }
        }
    }

    // Has a tendency to use all system memory
    fn quality_flood_fill(&mut self, position: [usize; 3], color: [f32; 3], intensity: u8, service: &mut ChunkService) {
        let mut points = Vec::new();
        let mut new_points = Vec::new();

        let mut current_intensity = intensity;

        points.push([
            position[0] as i32,
            position[1] as i32,
            position[2] as i32
        ]);

        while current_intensity != 0 {

            for pos in points.iter() {

                let col = current_intensity as f32 / 24.0;
                //let col = 1.0;

                // Add color to current points
                let new_color = [1.0, color[0], color[1], color[2]];
                apply_color_to_chunk(self, pos.clone(), new_color, col, service);

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

fn apply_color_to_chunk(chunk: &mut ChunkData, mut pos: [i32; 3], color: [f32; 4], intensity: f32, service: &mut ChunkService) {

    if pos[0] >= 0 && pos[0] <= 15 &&
        pos[1] >= 0 && pos[1] <= 15 &&
        pos[2] >= 0 && pos[2] <= 15 {

        // Its in current chunk.
        let current_color = &mut chunk.light_levels[pos[0] as usize][pos[1] as usize][pos[2] as usize];
        *current_color = lerp_color(color, *current_color, intensity);
    } else {
        // Its in other chunk
        let mut chunk_pos = chunk.position;

        // Calculate chunk position
        if pos[0] < 0 {
            chunk_pos.x -= 1;
            pos[0] = CHUNK_SIZE as i32 + pos[0];
        } else if pos[0] > 15 {
            chunk_pos.x += 1;
            pos[0] = ((2 * CHUNK_SIZE as i32) - 1) - pos[0];
        }

        if pos[1] < 0 {
            chunk_pos.y -= 1;
            pos[1] = CHUNK_SIZE as i32 + pos[1];
        } else if pos[0] > 15 {
            chunk_pos.y += 1;
            pos[1] = ((2 * CHUNK_SIZE as i32) - 1) - pos[1];
        }

        if pos[2] < 0 {
            chunk_pos.z -= 1;
            pos[2] = CHUNK_SIZE as i32 + pos[2];
        } else if pos[2] > 15 {
            chunk_pos.z += 1;
            pos[2] = ((2 * CHUNK_SIZE as i32) - 1) - pos[2];
        }

        let chunk = service.chunks.get_mut(&chunk_pos);

        // Make sure chunk exists
        if let Some(chunk) = chunk {
            // Make sure chunk has blocks to draw
            if let Chunk::Tangible(chunk) = chunk {
                let current_color = &mut chunk.neighboring_light_levels[pos[0] as usize][pos[1] as usize][pos[2] as usize];
                current_color[0] *= color[0];
                current_color[1] *= color[1];
                current_color[2] *= color[2];
                current_color[3] *= color[3];
                //println!("In: {:?} Chunk: {:?}", pos, chunk_pos);
            }
        }
    }
}