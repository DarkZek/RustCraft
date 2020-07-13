use crate::services::chunk_service::chunk::{ChunkData, Color, RawChunkData, RawLightingData};
use crate::services::settings_service::CHUNK_SIZE;
use crate::block::Block;
use nalgebra::{distance_squared, Point3, distance};

impl ChunkData {
    pub fn calculate_lighting(&mut self) {

        let mut lights = Vec::new();

        println!("Lights: {}", lights.len());

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
            println!("Pos: {:?}", light.2);
            flood_fill(&mut self.light_levels, &self.world, light.2, light.0, light.1);
        }
    }
}

fn flood_fill(lighting: &mut RawLightingData, world_context: &RawChunkData, position: [usize; 3], color: [f32; 3], intensity: u8) {
    let posx = position[0] as i8;
    let posy = position[1] as i8;
    let posz = position[2] as i8;

    for x in (position[0] as i8 - intensity as i8 .. posx + intensity as i8) {
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
                lighting[x as usize][y as usize][z as usize] = [color[0], color[1], color[2], light];
            }
        }
    }
}

// Alternate non working code
// fn flood_fill(lighting: &mut RawLightingData, world_context: &RawChunkData, position: [usize; 3], color: [f32; 3], intensity: u8) {
//     let mut points = Vec::new();
//     let mut new_points = Vec::new();
//
//     let mut current_intensity = intensity;
//
//     points.push(position);
//
//     while current_intensity != 0 {
//
//         for pos in points.iter() {
//             // Add color to current points
//             lighting[pos[0]][pos[1]][pos[2]] = [color[0], color[1], color[2], intensity as f32 / 14.0];
//
//             // Add adjacent tiles
//             if pos[0] < 15 { new_points.push([pos[0] + 1, pos[1], pos[2]]); }
//             if pos[0] != 0 { new_points.push([pos[0] - 1, pos[1], pos[2]]); }
//             if pos[1] < 15 { new_points.push([pos[0], pos[1] + 1, pos[2]]); }
//             if pos[1] != 0 { new_points.push([pos[0], pos[1] - 1, pos[2]]); }
//             if pos[2] < 15 { new_points.push([pos[0], pos[1], pos[2] + 1]); }
//             if pos[2] != 0 { new_points.push([pos[0], pos[1], pos[2] - 1]); }
//         }
//
//         points.clear();
//         points = new_points.clone();
//         new_points.clear();
//         current_intensity -= 1;
//     }
//     points.clear();
// }