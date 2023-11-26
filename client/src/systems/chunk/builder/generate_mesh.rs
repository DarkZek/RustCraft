use crate::game::viewable_direction::{ViewableDirection, BLOCK_SIDES};

use crate::systems::chunk::data::ChunkData;

use crate::systems::chunk::ChunkSystem;
use bevy::ecs::component::Component;

use crate::game::state::block::BlockStates;
use crate::helpers::global_to_local_position;
use crate::systems::chunk::mesh::draw_kit::DrawKit;
use crate::systems::chunk::nearby_cache::NearbyChunkCache;
use fnv::FnvHashMap;
use nalgebra::Vector3;
use rc_networking::constants::CHUNK_SIZE;

#[derive(Component)]
pub struct UpdateChunkMesh {
    pub chunk: Vector3<i32>,
    pub opaque: DrawKit,
    pub translucent: DrawKit,
    pub viewable_map: Option<[[[ViewableDirection; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
}

impl ChunkData {
    pub fn build_mesh(
        &self,
        chunks: &ChunkSystem,
        block_states: &BlockStates,
        edge_faces: bool,
        cache: &NearbyChunkCache,
    ) -> UpdateChunkMesh {
        // Get adjacent chunks
        let mut map = FnvHashMap::default();

        map.insert(
            Vector3::new(0, 1, 0),
            chunks.chunks.get(&(self.position + Vector3::new(0, 1, 0))),
        );
        map.insert(
            Vector3::new(0, -1, 0),
            chunks.chunks.get(&(self.position + Vector3::new(0, -1, 0))),
        );
        map.insert(
            Vector3::new(1, 0, 0),
            chunks.chunks.get(&(self.position + Vector3::new(1, 0, 0))),
        );
        map.insert(
            Vector3::new(-1, 0, 0),
            chunks.chunks.get(&(self.position + Vector3::new(-1, 0, 0))),
        );
        map.insert(
            Vector3::new(0, 0, 1),
            chunks.chunks.get(&(self.position + Vector3::new(0, 0, 1))),
        );
        map.insert(
            Vector3::new(0, 0, -1),
            chunks.chunks.get(&(self.position + Vector3::new(0, 0, -1))),
        );

        let viewable = self.generate_viewable_map(block_states, map, edge_faces);

        let mut opaque = DrawKit::new();
        let mut translucent = DrawKit::new();

        // Create the buffers to add the mesh data into
        let chunk = self.world;

        for x in 0..chunk.len() {
            for z in 0..chunk[0][0].len() {
                for y in 0..chunk[0].len() {
                    let viewable = viewable[x][y][z].0;

                    // Isn't air and is visible from at least one side
                    if chunk[x][y][z] != 0 && viewable != 0 {
                        let block = block_states.get_block(chunk[x][y][z] as usize);

                        let mut light_color = [self.light_levels[x][y][z]; 6];

                        for (i, side) in BLOCK_SIDES.iter().enumerate() {
                            let (chunk_pos, local_pos) = global_to_local_position(
                                Vector3::new(x, y, z).cast::<i32>()
                                    + side
                                    + (self.position * CHUNK_SIZE as i32),
                            );

                            light_color[i] = if let Some(chunk) = cache.get_chunk(chunk_pos) {
                                chunk.light_levels[local_pos.x][local_pos.y][local_pos.z]
                            } else {
                                [0; 4]
                            }
                        }

                        block.draw(
                            Vector3::new(x as f32, y as f32, z as f32),
                            ViewableDirection(viewable),
                            light_color,
                            if block.translucent {
                                &mut translucent
                            } else {
                                &mut opaque
                            },
                        );
                    }
                }
            }
        }

        // Check top faces
        UpdateChunkMesh {
            chunk: self.position,
            opaque,
            translucent,
            viewable_map: Some(viewable),
        }
    }
}
