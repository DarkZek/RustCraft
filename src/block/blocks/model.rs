use crate::services::asset_service::atlas::{TextureAtlasIndex, ATLAS_LOOKUPS};
use crate::services::chunk_service::chunk::Color;
use crate::services::chunk_service::mesh::culling::ViewableDirection;
use crate::services::chunk_service::mesh::{Vertex, ViewableDirectionBitMap};
use nalgebra::Vector3;
use std::ops::{Mul, Neg};

pub struct BlockModel {
    pub faces: Vec<BlockFace>,
}

pub struct BlockFace {
    pub bottom_left: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub texture: TextureAtlasIndex,
    pub normal: ViewableDirectionBitMap,
    // Is face on the edge of the block (used for culling)
    pub edge: bool,
}

impl BlockModel {
    pub fn square_block(textures: [&str; 6]) -> BlockModel {
        let mut faces = Vec::new();

        let mut face_textures = [TextureAtlasIndex::default(); 6];
        for i in 0..6 {
            match ATLAS_LOOKUPS.get().unwrap().get(textures[i]) {
                None => {
                    log_error!("No texture found for block with textures: {:?}", textures);
                    face_textures[i] = *ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap();
                }
                Some(texture) => face_textures[i] = *texture,
            }
        }

        // Top face
        faces.push(BlockFace {
            bottom_left: Vector3::new(0.0, 1.0, 0.0),
            scale: Vector3::new(1.0, 0.0, 1.0),
            texture: face_textures[0],
            normal: ViewableDirectionBitMap::Top,
            edge: true,
        });

        // Bottom face
        faces.push(BlockFace {
            bottom_left: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 0.0, 1.0),
            texture: face_textures[1],
            normal: ViewableDirectionBitMap::Bottom,
            edge: true,
        });

        // Left face
        faces.push(BlockFace {
            bottom_left: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(0.0, 1.0, 1.0),
            texture: face_textures[2],
            normal: ViewableDirectionBitMap::Left,
            edge: true,
        });

        // Right face
        faces.push(BlockFace {
            bottom_left: Vector3::new(1.0, 0.0, 0.0),
            scale: Vector3::new(0.0, 1.0, 1.0),
            texture: face_textures[3],
            normal: ViewableDirectionBitMap::Right,
            edge: true,
        });

        // Front face
        faces.push(BlockFace {
            bottom_left: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 0.0),
            texture: face_textures[4],
            normal: ViewableDirectionBitMap::Front,
            edge: true,
        });

        // Back face
        faces.push(BlockFace {
            bottom_left: Vector3::new(0.0, 0.0, 1.0),
            scale: Vector3::new(1.0, 1.0, 0.0),
            texture: face_textures[5],
            normal: ViewableDirectionBitMap::Back,
            edge: true,
        });

        BlockModel { faces }
    }

    pub fn draw(
        &self,
        x: f32,
        y: f32,
        z: f32,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u16>,
        applied_color: Color,
        viewable_map: ViewableDirection,
    ) {
        for face in &self.faces {
            // Test if we should cull this face
            if should_cull(face, viewable_map) {
                continue;
            }

            let atlas_index = face.texture.clone();

            let mut invert_normals = false;

            if face.normal == ViewableDirectionBitMap::Top
                || face.normal == ViewableDirectionBitMap::Back
                || face.normal == ViewableDirectionBitMap::Left
            {
                invert_normals = !invert_normals;
            }

            let mut normals: [f32; 3] = match face.normal {
                ViewableDirectionBitMap::Top => [0.0, 1.0, 0.0],
                ViewableDirectionBitMap::Front => [1.0, 0.0, 0.0],
                ViewableDirectionBitMap::Back => [-1.0, 0.0, 0.0],
                ViewableDirectionBitMap::Left => [0.0, 0.0, 1.0],
                ViewableDirectionBitMap::Right => [0.0, 0.0, -1.0],
                ViewableDirectionBitMap::Bottom => [0.0, -1.0, 0.0],
            };

            if invert_normals {
                normals[0] *= 1.0;
                normals[1] *= 1.0;
                normals[2] *= 1.0;
            }

            let starting_vertices = vertices.len() as u16;

            match face.normal {
                ViewableDirectionBitMap::Top | ViewableDirectionBitMap::Bottom => {
                    vertices.push(Vertex {
                        position: [
                            x + face.bottom_left[0],
                            y + face.bottom_left[1] + face.scale.y,
                            z + face.bottom_left[2],
                        ],
                        tex_coords: [atlas_index.u_max, atlas_index.v_max],
                        normals,
                        applied_color,
                    });
                    vertices.push(Vertex {
                        position: [
                            x + face.bottom_left[0],
                            y + face.bottom_left[1] + face.scale.y,
                            z + face.bottom_left[2] + face.scale.z,
                        ],
                        tex_coords: [atlas_index.u_max, atlas_index.v_min],
                        normals,
                        applied_color,
                    });
                    vertices.push(Vertex {
                        position: [
                            x + face.bottom_left[0] + face.scale.x,
                            y + face.bottom_left[1] + face.scale.y,
                            z + face.bottom_left[2] + face.scale.z,
                        ],
                        tex_coords: [atlas_index.u_min, atlas_index.v_min],
                        normals,
                        applied_color,
                    });
                    vertices.push(Vertex {
                        position: [
                            x + face.bottom_left[0] + face.scale.x,
                            y + face.bottom_left[1] + face.scale.y,
                            z + face.bottom_left[2],
                        ],
                        tex_coords: [atlas_index.u_min, atlas_index.v_max],
                        normals,
                        applied_color,
                    });
                }
                ViewableDirectionBitMap::Front
                | ViewableDirectionBitMap::Back
                | ViewableDirectionBitMap::Left
                | ViewableDirectionBitMap::Right => {
                    vertices.push(Vertex {
                        position: [
                            x + face.bottom_left[0] + face.scale.x,
                            y + face.bottom_left[1],
                            z + face.bottom_left[2] + face.scale.z,
                        ],
                        tex_coords: [atlas_index.u_min, atlas_index.v_max],
                        normals,
                        applied_color,
                    });
                    vertices.push(Vertex {
                        position: [
                            x + face.bottom_left[0] + face.scale.x,
                            y + face.bottom_left[1] + face.scale.y,
                            z + face.bottom_left[2] + face.scale.z,
                        ],
                        tex_coords: [atlas_index.u_min, atlas_index.v_min],
                        normals,
                        applied_color,
                    });
                    vertices.push(Vertex {
                        position: [
                            x + face.bottom_left[0],
                            y + face.bottom_left[1] + face.scale.y,
                            z + face.bottom_left[2],
                        ],
                        tex_coords: [atlas_index.u_max, atlas_index.v_min],
                        normals,
                        applied_color,
                    });
                    vertices.push(Vertex {
                        position: [
                            x + face.bottom_left[0],
                            y + face.bottom_left[1],
                            z + face.bottom_left[2],
                        ],
                        tex_coords: [atlas_index.u_max, atlas_index.v_max],
                        normals,
                        applied_color,
                    });
                }
            }

            if !invert_normals {
                indices.push(starting_vertices + 0);
                indices.push(starting_vertices + 1);
                indices.push(starting_vertices + 2);

                indices.push(starting_vertices + 0);
                indices.push(starting_vertices + 2);
                indices.push(starting_vertices + 3);
            } else {
                indices.push(starting_vertices + 0);
                indices.push(starting_vertices + 2);
                indices.push(starting_vertices + 1);

                indices.push(starting_vertices + 0);
                indices.push(starting_vertices + 3);
                indices.push(starting_vertices + 2);
            }
        }
    }
}

fn should_cull(block: &BlockFace, culling: ViewableDirection) -> bool {
    if culling.has_flag(&block.normal) || !block.edge {
        return false;
    }

    true
}
