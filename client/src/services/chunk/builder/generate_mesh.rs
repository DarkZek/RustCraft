use crate::game::mesh::draw_kit::DrawKit;
use crate::game::viewable_direction::{AxisAlignedDirection, ViewableDirection};

use crate::services::chunk::data::ChunkData;

use crate::services::chunk::ChunkService;
use bevy::ecs::component::Component;

use crate::game::blocks::states::BlockStates;
use fnv::FnvHashMap;
use nalgebra::Vector3;
use rc_networking::constants::CHUNK_SIZE;

#[derive(Component)]
pub struct UpdateChunkMesh {
    pub chunk: Vector3<i32>,
    pub positions: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub normals: Vec<[f32; 3]>,
    pub uv_coordinates: Vec<[f32; 2]>,
    pub lighting: Vec<[f32; 4]>,
    pub viewable_map: Option<[[[ViewableDirection; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
}

impl ChunkData {
    pub fn build_mesh(
        &self,
        chunks: &ChunkService,
        block_states: &BlockStates,
        edge_faces: bool,
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

        let mut positions = Vec::new();
        let mut indices = Vec::new();
        let mut normals = Vec::new();
        let mut uv_coordinates = Vec::new();
        let mut lighting = Vec::new();

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

                        light_color[AxisAlignedDirection::Top as usize] = if y < CHUNK_SIZE - 1 {
                            self.light_levels[x][y + 1][z]
                        } else {
                            [0; 4]
                        };
                        light_color[AxisAlignedDirection::Bottom as usize] = if y > 0 {
                            self.light_levels[x][y - 1][z]
                        } else {
                            [0; 4]
                        };

                        light_color[AxisAlignedDirection::Right as usize] = if x < CHUNK_SIZE - 1 {
                            self.light_levels[x + 1][y][z]
                        } else {
                            [0; 4]
                        };
                        light_color[AxisAlignedDirection::Left as usize] = if x > 0 {
                            self.light_levels[x - 1][y][z]
                        } else {
                            [0; 4]
                        };

                        light_color[AxisAlignedDirection::Back as usize] = if z < CHUNK_SIZE - 1 {
                            self.light_levels[x][y][z + 1]
                        } else {
                            [0; 4]
                        };
                        light_color[AxisAlignedDirection::Front as usize] = if z > 0 {
                            self.light_levels[x][y][z - 1]
                        } else {
                            [0; 4]
                        };

                        block.draw(
                            Vector3::new(x as f32, y as f32, z as f32),
                            ViewableDirection(viewable),
                            light_color,
                            DrawKit {
                                positions: &mut positions,
                                indices: &mut indices,
                                normals: &mut normals,
                                uv_coordinates: &mut uv_coordinates,
                                lighting: &mut lighting,
                            },
                        );
                    }
                }
            }
        }

        // Check top faces
        UpdateChunkMesh {
            chunk: self.position,
            positions,
            indices,
            normals,
            uv_coordinates,
            lighting,
            viewable_map: Some(viewable),
        }
    }
}
