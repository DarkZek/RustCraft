use crate::systems::chunk::data::ChunkData;
use nalgebra::Vector3;
use rc_shared::chunk::{RawLightingData};
use rc_shared::viewable_direction::BLOCK_SIDES;
use rc_shared::{CHUNK_SIZE};
use std::collections::VecDeque;
use crate::systems::chunk::builder::build_context::ChunkBuildContext;
use rc_shared::relative_chunk_map::RelativeChunkMap;

const MAX_SKYLIGHT_BRIGHTNESS: usize = 12;

impl ChunkData {
    pub fn build_skylighting(
        &self,
        context: &mut ChunkBuildContext,
        lighting_data: &mut RawLightingData
    ) {

        if context.sunlight_points.len() == 0 {
            return;
        }

        // Rolling average of chunk lighting data
        let mut data: RelativeChunkMap<u8> = RelativeChunkMap::new_empty(
            self.position * CHUNK_SIZE as i32,
            1
        );

        let mut point = VecDeque::with_capacity(1000);

        // Propagate lighting
        for light_pos in &context.sunlight_points {
            point.push_back((*light_pos, MAX_SKYLIGHT_BRIGHTNESS as u8));
        }

        while !point.is_empty() {
            let (pos, strength) = point.pop_front().unwrap();

            if context.translucency_map.get_position(pos).is_none() ||
                !context.translucency_map.get_position(pos).unwrap() {
                // Collision, bail
                continue;
            }

            if let Some(current_strength) = data.get_mut(pos.into()) {
                if *current_strength > strength {
                    continue;
                } else {
                    data.set(pos.into(), strength);
                }
            } else {
                continue;
            }

            if strength == 1 {
                continue;
            }

            for side in &BLOCK_SIDES {
                point.push_back((pos + side, strength - 1));
            }
        }

        // Apply to `lighting_data`
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let light_strength = *data.get(
                        self.position * CHUNK_SIZE as i32 + Vector3::new(x as i32, y as i32, z as i32)
                    ).unwrap();
                    lighting_data[x][y][z].skylight = light_strength;
                }
            }
        }

        // Update surrounding blocks
        for (pos, block_data) in &mut context.surrounding_data {
            // Get data
            if let Some(light_data) = data.get(*pos) {
                block_data.light.skylight = *light_data;
            }
        }
    }
}
