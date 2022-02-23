use crate::services::asset_service::atlas::ATLAS_LOOKUPS;
use crate::services::asset_service::index::TextureAtlasIndex;
use crate::services::chunk_service::chunk::Color;
use crate::services::chunk_service::mesh::culling::ViewableDirection;
use crate::services::chunk_service::mesh::{Vertex, ViewableDirectionBitMap};
use nalgebra::Vector3;
use std::f32::consts::PI;
use std::ops::Mul;

#[derive(Copy, Clone)]
pub enum Rotate {
    Deg90,
    Deg180,
    Deg270,
}

pub struct BlockModel {
    pub faces: Vec<BlockFace>,
}

#[derive(Copy, Clone)]
pub struct BlockFace {
    pub bottom_left: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub texture: TextureAtlasIndex,
    pub normal: ViewableDirectionBitMap,
    pub color: [u8; 4],
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
            color: [255; 4],
            edge: true,
        });

        // Bottom face
        faces.push(BlockFace {
            bottom_left: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 0.0, 1.0),
            texture: face_textures[1],
            normal: ViewableDirectionBitMap::Bottom,
            color: [255; 4],
            edge: true,
        });

        // Left face
        faces.push(BlockFace {
            bottom_left: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(0.0, 1.0, 1.0),
            texture: face_textures[2],
            normal: ViewableDirectionBitMap::Left,
            color: [255; 4],
            edge: true,
        });

        // Right face
        faces.push(BlockFace {
            bottom_left: Vector3::new(1.0, 0.0, 0.0),
            scale: Vector3::new(0.0, 1.0, 1.0),
            texture: face_textures[3],
            normal: ViewableDirectionBitMap::Right,
            color: [255; 4],
            edge: true,
        });

        // Front face
        faces.push(BlockFace {
            bottom_left: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 0.0),
            texture: face_textures[4],
            normal: ViewableDirectionBitMap::Front,
            color: [255; 4],
            edge: true,
        });

        // Back face
        faces.push(BlockFace {
            bottom_left: Vector3::new(0.0, 0.0, 1.0),
            scale: Vector3::new(1.0, 1.0, 0.0),
            texture: face_textures[5],
            normal: ViewableDirectionBitMap::Back,
            color: [255; 4],
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
            if face.should_cull(viewable_map) {
                continue;
            }

            let atlas_index = face.texture.clone();

            let mut normals: [f32; 3] = match face.normal {
                ViewableDirectionBitMap::Top => [0.0, 1.0, 0.0],
                ViewableDirectionBitMap::Front => [1.0, 0.0, 0.0],
                ViewableDirectionBitMap::Back => [-1.0, 0.0, 0.0],
                ViewableDirectionBitMap::Left => [0.0, 0.0, 1.0],
                ViewableDirectionBitMap::Right => [0.0, 0.0, -1.0],
                ViewableDirectionBitMap::Bottom => [0.0, -1.0, 0.0],
            };

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

            if face.normal == ViewableDirectionBitMap::Top
                || face.normal == ViewableDirectionBitMap::Back
                || face.normal == ViewableDirectionBitMap::Left
            {
                indices.push(starting_vertices + 0);
                indices.push(starting_vertices + 2);
                indices.push(starting_vertices + 1);

                indices.push(starting_vertices + 0);
                indices.push(starting_vertices + 3);
                indices.push(starting_vertices + 2);
            } else {
                indices.push(starting_vertices + 0);
                indices.push(starting_vertices + 1);
                indices.push(starting_vertices + 2);

                indices.push(starting_vertices + 0);
                indices.push(starting_vertices + 2);
                indices.push(starting_vertices + 3);
            }
        }
    }

    pub fn rotate_xz(&mut self, deg: Rotate) {
        for face in &mut self.faces {
            face.rotate(deg);
        }
    }

    pub fn invert_y(&mut self) {
        for face in &mut self.faces {
            face.bottom_left.y = 1.0 - face.bottom_left.y;
            face.scale.y = -face.scale.y;
            face.normal = face.normal.invert();
            face.texture = face.texture.invert();
        }
    }
}

impl BlockFace {
    fn should_cull(&self, culling: ViewableDirection) -> bool {
        if culling.has_flag(&self.normal) || !self.edge {
            return false;
        }

        true
    }

    pub fn rotate(&mut self, deg: Rotate) {
        // Move to center being 0,0,0
        self.bottom_left += Vector3::new(-0.5, -0.5, -0.5);

        let rot = match deg {
            Rotate::Deg90 => PI * 0.5,
            Rotate::Deg180 => PI,
            Rotate::Deg270 => PI * 1.5,
        };

        let rotation_vector = nalgebra::geometry::Rotation3::from_euler_angles(0.0, rot, 0.0);
        let rotation_matrix = rotation_vector.matrix();
        self.bottom_left = rotation_matrix.mul(&self.bottom_left);
        self.scale = rotation_matrix.mul(&self.scale);

        self.texture.rotate(deg);

        // Move back
        self.bottom_left += Vector3::new(0.5, 0.5, 0.5);

        self.normal = self.normal.rotate(deg);
    }
}
