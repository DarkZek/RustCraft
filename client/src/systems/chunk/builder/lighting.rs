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

#[derive(Copy, Clone, Debug, Default)]
pub struct BlockLightRecord {
    weighted_cumulative_r: u32,
    weighted_cumulative_g: u32,
    weighted_cumulative_b: u32,
    cumulative_strength: u32,
    max_strength: u8
}

impl ChunkData {
    pub fn build_lighting(
        &self,
        context: LightingContext
    ) -> LightingUpdateData {

        let start = Instant::now();

        if context.lights.len() == 0 {
            return LightingUpdateData {
                data: [[[[0 as u8; 4]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            };
        }

        let lights_len = context.lights.len();

        // Rolling average of chunk lighting data
        let mut data = [[[BlockLightRecord::default(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        // Propagate lighting
        for (light_pos, color) in &context.lights {
            // a 32 block wide area around the light that tracks what blocks it's visited
            let mut visited = [[[false; CHUNK_SIZE * 2]; CHUNK_SIZE * 2]; CHUNK_SIZE * 2];

            let mut point = VecDeque::with_capacity(1000);

            // Starting points
            point.push_back((light_pos + Vector3::new(1, 0, 0), color[3] - 1));
            point.push_back((light_pos + Vector3::new(-1, 0, 0), color[3] - 1));
            point.push_back((light_pos + Vector3::new(0, 1, 0), color[3] - 1));
            point.push_back((light_pos + Vector3::new(0, -1, 0), color[3] - 1));
            point.push_back((light_pos + Vector3::new(0, 0, 1), color[3] - 1));
            point.push_back((light_pos + Vector3::new(0, 0, -1), color[3] - 1));

            while !point.is_empty() {
                let (pos, strength) = point.pop_front().unwrap();

                let (chunk_pos, block_pos) = global_to_local_position(pos);

                if visited[(pos.x - light_pos.x + CHUNK_SIZE as i32) as usize]
                    [(pos.y - light_pos.y + CHUNK_SIZE as i32) as usize]
                    [(pos.z - light_pos.z + CHUNK_SIZE as i32) as usize]
                {
                    continue;
                }

                let local_relative_position =
                    pos - (CHUNK_SIZE as i32 * chunk_pos)
                        + Vector3::new(CHUNK_SIZE as i32, CHUNK_SIZE as i32, CHUNK_SIZE as i32);

                if !context.translucency_map
                    [local_relative_position.x as usize]
                    [local_relative_position.y as usize]
                    [local_relative_position.z as usize] {
                    // Collision, bail
                    continue;
                }

                if chunk_pos == self.position {
                    let current_color =
                        &mut data[block_pos.x][block_pos.y][block_pos.z];
                    current_color.weighted_cumulative_r += strength as u32 * color[0] as u32;
                    current_color.weighted_cumulative_g += strength as u32 * color[1] as u32;
                    current_color.weighted_cumulative_b += strength as u32 * color[2] as u32;
                    current_color.cumulative_strength += strength as u32;
                    current_color.max_strength = current_color.max_strength.max(strength);
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
                    if color.max_strength == 0 {
                        continue;
                    }

                    let out_color = &mut out[x][y][z];

                    // Get the proportional color
                    out_color[0] = (color.weighted_cumulative_r / color.cumulative_strength) as u8;
                    out_color[1] = (color.weighted_cumulative_g / color.cumulative_strength) as u8;
                    out_color[2] = (color.weighted_cumulative_b / color.cumulative_strength) as u8;

                    // Light falloff based off max strength
                    out_color[0] =
                        (out_color[0] as u32 * color.max_strength as u32 / MAX_LIGHT_VALUE as u32) as u8;
                    out_color[1] =
                        (out_color[1] as u32 * color.max_strength as u32 / MAX_LIGHT_VALUE as u32) as u8;
                    out_color[2] =
                        (out_color[2] as u32 * color.max_strength as u32 / MAX_LIGHT_VALUE as u32) as u8;

                    out_color[3] = color.max_strength;
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

pub struct LightingContext {
    // Location of all lights in the scene
    lights: Vec<(Vector3<i32>, [u8; 4])>,
    // Translucency of all blocks in the surrounding chunks.
    // TODO: Convert this into a more compressed format?
    translucency_map: [[[bool; CHUNK_SIZE * 3]; CHUNK_SIZE * 3]; CHUNK_SIZE * 3]
}

// Stores any context a chunk may need to build lighting. Used so that chunk can be chucked at another thread
impl LightingContext {
    pub(crate) fn new(
        chunk_pos: Vector3<i32>,
        states: &BlockStates,
        cache: &NearbyChunkCache,
    ) -> LightingContext {
        let mut lights = Vec::new();

        let mut translucency_map = [[[false; 48]; 48]; 48];

        for chunk_x in (chunk_pos.x - 1)..=(chunk_pos.x + 1) {
            for chunk_y in (chunk_pos.y - 1)..=(chunk_pos.y + 1) {
                for chunk_z in (chunk_pos.z - 1)..=(chunk_pos.z + 1) {
                    // Get chunk
                    if let Some(chunk) = cache.get_chunk(Vector3::new(chunk_x, chunk_y, chunk_z)) {
                        for x in 0..CHUNK_SIZE {
                            for y in 0..CHUNK_SIZE {
                                for z in 0..CHUNK_SIZE {
                                    let block = states.get_block(chunk.world[x][y][z] as usize);

                                    let block_position = Vector3::new(x, y, z).cast::<i32>()
                                        + (CHUNK_SIZE as i32 * chunk.position);

                                    // Position in 3x3 array of chunk data
                                    let local_relative_position =
                                        block_position - (CHUNK_SIZE as i32 * chunk_pos)
                                            + Vector3::new(CHUNK_SIZE as i32, CHUNK_SIZE as i32, CHUNK_SIZE as i32);

                                    translucency_map
                                        [local_relative_position.x as usize]
                                        [local_relative_position.y as usize]
                                        [local_relative_position.z as usize] = block.translucent;

                                    if block.emission[3] == 0 {
                                        continue;
                                    }

                                    lights.push((
                                        block_position,
                                        block.emission,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        LightingContext {
            lights,
            translucency_map,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs};
    use std::time::Instant;
    use bevy_inspector_egui::egui::ahash::HashMapExt;
    use fnv::FnvHashMap;
    use rc_shared::block::BlockStates;
    use rc_shared::block::types::Block;
    use crate::systems::chunk::builder::lighting::{LightingContext};
    use crate::systems::chunk::data::ChunkData;
    use crate::systems::chunk::nearby_cache::NearbyChunkCache;
    use crate::systems::chunk::static_world_data::StaticWorldData;

    #[test]
    fn benchmark_chunk_building() {

        let file_data = fs::read("chunk_lighting_benchmark.mpk").unwrap();
        let world_data = rmp_serde::from_slice::<StaticWorldData>(file_data.as_slice()).unwrap();

        let mut chunks = FnvHashMap::new();

        for chunk_data in world_data.data {
            let chunk = ChunkData::new_handleless(chunk_data.data, chunk_data.position);
            chunks.insert(chunk.position, chunk);
        }

        let mut states = BlockStates::new();

        states.states.push(Block {
            identifier: "mcv3::Air".to_string(),
            translucent: true,
            full: false,
            draw_betweens: false,
            faces: vec![],
            collision_boxes: vec![],
            bounding_boxes: vec![],
            emission: [0; 4],
        });

        for i in 0..7 {
            states.states.push(Block {
                identifier: "mcv3::Stone".to_string(),
                translucent: false,
                full: true,
                draw_betweens: false,
                faces: vec![],
                collision_boxes: vec![],
                bounding_boxes: vec![],
                emission: [0; 4],
            });
        }
        states.states.push(Block {
            identifier: "mcv3::Lamp".to_string(),
            translucent: true,
            full: true,
            draw_betweens: false,
            faces: vec![],
            collision_boxes: vec![],
            bounding_boxes: vec![],
            emission: [255, 255, 255, 16],
        });


        for i in 0..10 {
            let mut total_time_nanos = 0;

            for (pos, chunk) in &chunks {
                let nearby_block_cache = NearbyChunkCache::from_map(&chunks, *pos);

                let start = Instant::now();

                let context = LightingContext::new(*pos, &states, &nearby_block_cache);

                let translucency_map = [[[false; 48]; 48]; 48];

                chunk.build_lighting(context);
                total_time_nanos += start.elapsed().as_nanos();
            }

            println!("Took {}ms per chunk", total_time_nanos as f32 / 1000000.0 / chunks.len() as f32);
        }

    }
}