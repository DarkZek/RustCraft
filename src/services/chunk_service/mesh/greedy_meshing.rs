use crate::services::chunk_service::chunk::Chunk;
use crate::services::chunk_service::mesh::{Vertex, ViewableDirectionBitMap};
use crate::services::settings_service::{SettingsService};
use crate::services::chunk_service::mesh::culling::{calculate_viewable, ViewableDirection};
use crate::services::chunk_service::mesh::block::{draw_block, draw_vertical_face};
use cgmath::{Point3, Vector3};
use crate::services::asset_service::atlas::TextureAtlasIndex;
use crate::block::BlockDirection;

//
// Our greedy meshing system
//

#[derive(Clone, Copy, Debug)]
struct Face {
    start: Point3<f32>,
    size: Vector3<f32>,
    atlas: TextureAtlasIndex
}

impl Face {
    pub fn new(start: Point3<f32>, atlas: TextureAtlasIndex) -> Face{
        Face {
            start,
            size: Vector3 { x: 1.0, y: 1.0, z: 1.0 },
            atlas,
        }
    }
}

impl Chunk {

    pub fn generate_mesh(&mut self, settings: &SettingsService) {
        self.generate_viewable_map();
        let (mut vertices, mut indices) = self.calculate_vertical_meshes();

        // Create the buffers to add the mesh data into
        let world = self.world;

        for x in 0..world.len() {
            for z in 0..world[0][0].len() {
                for y in 0..world[0].len() {
                    let viewable = calculate_viewable(&self, [x, y, z]);

                    //Isn't air
                    if world[x][y][z] != 0 && viewable != 0 {
                        let block = &self.blocks[world[x][y][z] as usize - 1];

                        //Found it, draw vertices for it
                        if !settings.debug_vertices {
                            draw_block(Point3 {
                                x: x as f32,
                                y: y as f32,
                                z: z as f32
                            }, ViewableDirection(viewable), &mut vertices, &mut indices, block);
                        } else {
                            //TODO: DRAW BLOCK WITH VERTICES OUTLINE
                        }
                    }
                }
            }
        }

        self.vertices = Some(vertices);
        self.indices = Some(indices);
    }

    pub fn calculate_vertical_meshes(&mut self) -> (Vec<Vertex>, Vec<u16>) {
        let mut vertices: Vec<Vertex> = Vec::with_capacity(10_000);
        let mut indices: Vec<u16> = Vec::with_capacity(10_000);
        let world = &self.world;
        let viewable_map = &self.viewable_map.unwrap();

        let mut faces_to_draw: Vec<Face> = Vec::new();

        for y in 0..world[0].len() {

            // The faces that need to be drawn for this pane
            let mut pane_faces: Vec<Face> = Vec::new();

            for x in 0..world.len() {

                // The faces that need to be drawn for this row
                let mut row_faces: Vec<Face> = Vec::new();

                for z in 0..world[0][0].len() {
                    let viewable = &viewable_map[x][y][z];

                    //Isn't air and its viewable from the top
                    if world[x][y][z] != 0 && viewable.has_flag(ViewableDirectionBitMap::Top) {
                        // Check if we can use the same face as the previous block
                        if z != 0 && self.blocks.get(world[x][y][z - 1] as usize).unwrap().id == self.blocks.get(world[x][y][z] as usize).unwrap().id {
                            let len = row_faces.len() - 1;
                            let face = row_faces.get_mut( len).unwrap();
                            face.size.z += 1.0;
                        } else {
                            let texture: TextureAtlasIndex = self.blocks.get(world[x][y][z] as usize - 1).unwrap().texture_atlas_lookups[BlockDirection::Up as usize];

                            // Create a new face, it's only one block so we don't set the size yet
                            row_faces.push(Face::new(Point3 {
                                x: x as f32,
                                y: y as f32,
                                z: z as f32
                            }, texture));
                        }
                    }
                }

                // Combines the Z row panes into cubes (if they can fit)
                let mut culled_faces = Vec::new();

                for mut face in pane_faces.iter_mut() {
                    // Check if they were in the previous row
                    if face.start.x + face.size.x != x as f32 {
                        continue;
                    }

                    for current_row_face in &row_faces {
                        // Check if they're the same position and size
                        if current_row_face.start.z == face.start.z && current_row_face.size.z == face.size.z && face.atlas == current_row_face.atlas {
                            face.size.x += 1.0;
                            culled_faces.push(current_row_face.start);
                            break;
                        }
                    }
                }

                // Add all faces that have'nt been culled
                for face in row_faces {
                    let mut removed = false;

                    for culled in &culled_faces {
                        if &face.start == culled {
                            removed = true;
                        }
                    }

                    if !removed {
                        pane_faces.push(face);
                    }
                }
            }

            faces_to_draw.append(&mut pane_faces);
        }

        for mut face in faces_to_draw {

            // Somewhere along they chain they're moved up one too high
            face.start.y -= 1.0;

            draw_vertical_face(face.start, face.size, &mut vertices, &mut indices, true, face.atlas);
            //println!("Face: {:?} {:?}", face.start, face.size);
        }

        (vertices, indices)

    }

}