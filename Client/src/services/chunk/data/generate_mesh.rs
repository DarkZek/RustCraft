use crate::game::blocks::BlockStates;
use crate::game::mesh::draw_kit::DrawKit;
use crate::game::viewable_direction::ViewableDirection;
use crate::services::asset::atlas::index::TextureAtlasIndex;
use crate::services::chunk::data::ChunkData;
use crate::services::chunk::lookup::Chunks;
use crate::services::chunk::ChunkService;
use bevy::ecs::component::Component;
use bevy::log::error;
use bevy::render::render_resource::BindGroup;
use fnv::FnvHashMap;
use nalgebra::{Vector2, Vector3};
use rustcraft_protocol::constants::CHUNK_SIZE;

#[derive(Component)]
pub struct UpdateChunkMesh {
    pub chunk: Vector3<i32>,
    pub positions: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub normals: Vec<[f32; 3]>,
    pub uv_coordinates: Vec<[f32; 2]>,
    pub viewable_map: Option<[[[ViewableDirection; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
}

impl ChunkData {
    pub fn generate_mesh(
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

        // Create the buffers to add the mesh data into
        let chunk = self.world;

        for x in 0..chunk.len() {
            for z in 0..chunk[0][0].len() {
                for y in 0..chunk[0].len() {
                    let viewable = viewable[x][y][z].0;

                    // Isn't air and is visible from at least one side
                    if chunk[x][y][z] != 0 && viewable != 0 {
                        let block = match block_states.get_block(chunk[x][y][z] as usize) {
                            None => {
                                error!(
                                    "Block with invalid blockstate: X {} Y {} Z {} Block ID {}",
                                    x, y, z, chunk[x][y][z]
                                );
                                continue;
                            }
                            Some(block) => block,
                        };

                        let light_color = self.light_levels[x][y][z];

                        block.draw(
                            Vector3::new(x as f32, y as f32, z as f32),
                            ViewableDirection(viewable),
                            DrawKit {
                                positions: &mut positions,
                                indices: &mut indices,
                                normals: &mut normals,
                                uv_coordinates: &mut uv_coordinates,
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
            viewable_map: Some(viewable),
        }
    }
}
