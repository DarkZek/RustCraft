use crate::systems::chunk::data::ChunkData;
use bevy::log::{debug};
use nalgebra::Vector3;
use rc_shared::chunk::{LightingColor, RawLightingData};
use rc_shared::viewable_direction::BLOCK_SIDES;
use rc_shared::{CHUNK_SIZE, MAX_LIGHT_VALUE};
use std::collections::VecDeque;
use serde::{Deserialize, Serialize};
use web_time::Instant;
use crate::systems::chunk::builder::build_context::ChunkBuildContext;
use rc_shared::relative_chunk_map::RelativeChunkMap;

#[derive(Serialize, Deserialize)]
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
        context: &mut ChunkBuildContext
    ) -> LightingUpdateData {

        let start = Instant::now();

        if context.lights.len() == 0 {
            return LightingUpdateData {
                data: [[[LightingColor::default(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            };
        }

        let lights_len = context.lights.len();

        // Rolling average of chunk lighting data
        let mut data: RelativeChunkMap<BlockLightRecord> = RelativeChunkMap::new_empty(
            self.position * CHUNK_SIZE as i32,
            1
        );

        // Propagate lighting
        for (light_pos, color) in &context.lights {
            // a 32 block wide area around the light that tracks what blocks it's visited
            // TODO: Decrease size
            let mut visited = [[[false; CHUNK_SIZE * 2]; CHUNK_SIZE * 2]; CHUNK_SIZE * 2];

            let mut point = VecDeque::with_capacity(1000);

            // Starting points
            point.push_back((light_pos + Vector3::new(1, 0, 0),     color[3] - 1));
            point.push_back((light_pos + Vector3::new(-1, 0, 0),    color[3] - 1));
            point.push_back((light_pos + Vector3::new(0, 1, 0),     color[3] - 1));
            point.push_back((light_pos + Vector3::new(0, -1, 0),    color[3] - 1));
            point.push_back((light_pos + Vector3::new(0, 0, 1),     color[3] - 1));
            point.push_back((light_pos + Vector3::new(0, 0, -1),    color[3] - 1));

            while !point.is_empty() {
                let (pos, strength) = point.pop_front().unwrap();

                let light_index = Vector3::new(
                    (pos.x - light_pos.x + CHUNK_SIZE as i32) as usize,
                        (pos.y - light_pos.y + CHUNK_SIZE as i32) as usize,
                        (pos.z - light_pos.z + CHUNK_SIZE as i32) as usize
                );

                if visited[light_index.x][light_index.y][light_index.z]
                {
                    continue;
                }

                if context.translucency_map.get_position(pos).is_none() ||
                    !context.translucency_map.get_position(pos).unwrap() {
                    // Collision, bail
                    continue;
                }

                if let Some(current_color) = data.get_mut(pos.into()) {
                    current_color.weighted_cumulative_r += strength as u32 * color[0] as u32;
                    current_color.weighted_cumulative_g += strength as u32 * color[1] as u32;
                    current_color.weighted_cumulative_b += strength as u32 * color[2] as u32;
                    current_color.cumulative_strength += strength as u32;
                    current_color.max_strength = current_color.max_strength.max(strength);
                }

                visited[light_index.x][light_index.y][light_index.z] = true;

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
        let mut out = [[[LightingColor::default(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let color = data.get(
                        self.position * CHUNK_SIZE as i32 + Vector3::new(x as i32, y as i32, z as i32)
                    ).unwrap();

                    // If there's no lighting data for this block ignore it
                    if color.max_strength == 0 {
                        continue;
                    }

                    out[x][y][z] = calculate_color(&color);
                }
            }
        }

        // Update context surrounding blocks with results of lighting pass
        for (pos, entry) in &mut context.surrounding_data {
            if let Some(lighting_data) = data.get(*pos) {
                // If there's no lighting data for this block ignore it
                if lighting_data.max_strength == 0 {
                    continue;
                }

                entry.light = calculate_color(lighting_data);
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

fn calculate_color(color: &BlockLightRecord) -> LightingColor {
    let mut out_color = LightingColor {
        r: (color.weighted_cumulative_r / color.cumulative_strength) as u8,
        g: (color.weighted_cumulative_g / color.cumulative_strength) as u8,
        b: (color.weighted_cumulative_b / color.cumulative_strength) as u8,
        strength: color.max_strength,
        skylight: 0,
    };

    // Light falloff based off max strength
    out_color.r =
        (out_color.r as u32 * color.max_strength as u32 / MAX_LIGHT_VALUE as u32) as u8;
    out_color.g =
        (out_color.g as u32 * color.max_strength as u32 / MAX_LIGHT_VALUE as u32) as u8;
    out_color.b =
        (out_color.b as u32 * color.max_strength as u32 / MAX_LIGHT_VALUE as u32) as u8;

    out_color
}

#[cfg(test)]
mod tests {
    use std::{fs};
    use web_time::Instant;
    use fnv::FnvHashMap;
    use nalgebra::Vector2;
    use rc_shared::block::BlockStates;
    use rc_shared::block::types::VisualBlock;
    use crate::systems::chunk::builder::build_context::ChunkBuildContext;
    use crate::systems::chunk::data::ChunkData;
    use crate::systems::chunk::nearby_cache::NearbyChunkCache;
    use crate::systems::chunk::nearby_column_cache::NearbyChunkColumnCache;
    use crate::systems::chunk::static_world_data::StaticWorldData;

    #[test]
    fn benchmark_chunk_building() {

        let file_data = fs::read("chunk_lighting_benchmark.mpk").unwrap();
        let world_data = rmp_serde::from_slice::<StaticWorldData>(file_data.as_slice()).unwrap();

        let mut chunks = FnvHashMap::default();

        for chunk_data in world_data.data {
            let chunk = ChunkData::new_handleless(chunk_data.data, chunk_data.position);
            chunks.insert(chunk.position, chunk);
        }

        let mut states = BlockStates::new();

        states.states.push(VisualBlock {
            identifier: "mcv3::Air".to_string(),
            translucent: true,
            full: false,
            draw_betweens: false,
            faces: vec![],
            collision_boxes: vec![],
            bounding_boxes: vec![],
            emission: [0; 4],
        });

        for _ in 0..7 {
            states.states.push(VisualBlock {
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
        states.states.push(VisualBlock {
            identifier: "mcv3::Lamp".to_string(),
            translucent: true,
            full: true,
            draw_betweens: false,
            faces: vec![],
            collision_boxes: vec![],
            bounding_boxes: vec![],
            emission: [255, 255, 255, 16],
        });


        for _ in 0..10 {
            let mut total_time_nanos = 0;

            for (pos, chunk) in &chunks {
                let nearby_block_cache = NearbyChunkCache::from_map(&chunks, *pos);
                let nearby_column_cache = NearbyChunkColumnCache::empty(Vector2::new(pos.x, pos.z));

                let start = Instant::now();

                let mut context = ChunkBuildContext::new(&states, &nearby_block_cache, &nearby_column_cache);

                chunk.build_lighting(&mut context);
                total_time_nanos += start.elapsed().as_nanos();
            }

            println!("Took {}ms per chunk", total_time_nanos as f32 / 1000000.0 / chunks.len() as f32);
        }

    }
}