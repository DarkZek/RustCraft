use crate::game::block::Draw;
use crate::systems::chunk::data::ChunkData;
use bevy::ecs::component::Component;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use rc_shared::block::BlockStates;
use rc_shared::chunk::LightingColor;
use rc_shared::helpers::global_to_local_position;
use rc_shared::viewable_direction::{ViewableDirection, BLOCK_SIDES};
use rc_shared::CHUNK_SIZE;
use crate::systems::chunk::builder::build_context::ChunkBuildContext;
use crate::utils::mesh::draw_kit::DrawKit;

#[derive(Component, Serialize, Deserialize)]
pub struct UpdateChunkMesh {
    pub chunk: Vector3<i32>,
    pub opaque: DrawKit,
    pub translucent: DrawKit,
    pub viewable_map: Option<[[[ViewableDirection; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
}

impl ChunkData {
    pub fn build_mesh(
        &self,
        block_states: &BlockStates,
        edge_faces: bool,
        context: &ChunkBuildContext,
    ) -> UpdateChunkMesh {

        let viewable = self.generate_viewable_map(block_states, context, edge_faces);

        let mut opaque = DrawKit::new();
        let mut translucent = DrawKit::new().with_wind_strength();

        // Create the buffers to add the mesh data into
        let chunk = &self.world;

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let viewable = viewable[x][y][z].0;
                    let pos = Vector3::new(x, y, z);

                    let block_id = chunk.get(pos);

                    // Isn't air and is visible from at least one side
                    if block_id != 0 && viewable != 0 {
                        let block = block_states.get_block_from_id(block_id);

                        let mut light_color = [self.light_levels[x][y][z]; 6];

                        for (i, side) in BLOCK_SIDES.iter().enumerate() {
                            let world_position = Vector3::new(x, y, z).cast::<i32>()
                                + side
                                + (self.position * CHUNK_SIZE as i32);

                            let (chunk_pos, local_pos) = global_to_local_position(world_position);

                            if chunk_pos == self.position {
                                light_color[i] = self.light_levels[local_pos.x][local_pos.y][local_pos.z];
                                continue;
                            }

                            light_color[i] = if let Some(chunk) = context.surrounding_data.get(&world_position) {
                                chunk.light
                            } else {
                                LightingColor::default()
                            }
                        }

                        let visual_block = block.draw();
                        visual_block.draw(
                            Vector3::new(x as f32, y as f32, z as f32),
                            ViewableDirection(viewable),
                            light_color,
                            if visual_block.translucent {
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
