use wgpu::{Device, BindGroupLayout};
use crate::services::chunk_service::mesh::culling::{calculate_viewable, ViewableDirection};
use crate::services::chunk_service::chunk::Chunk;
use crate::services::settings_service::{CHUNK_SIZE};
use std::collections::HashMap;
use crate::block::Block;
use crate::services::chunk_service::mesh::{ViewableDirectionBitMap, Vertex};
use nalgebra::{Matrix4, Vector3};

pub struct ChunkMeshData {
    pub viewable: Option<[[[ViewableDirection; 16]; 16]; 16]>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>
}

impl<'a> Chunk {

    pub fn create_buffers(&mut self, device: &Device, bind_group_layout: &BindGroupLayout) {
        let vertices = self.vertices.as_ref().unwrap();

        let vertex_buffer = device
            .create_buffer_mapped(vertices.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(vertices.as_slice());

        self.vertices_buffer = Some(vertex_buffer);

        let indices = self.indices.take().unwrap();
        self.indices_buffer_len = indices.len() as u32;

        let indices_buffer = device
            .create_buffer_mapped(indices.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(indices.as_slice());

        self.indices_buffer = Some(indices_buffer);

        // Create model buffer
        let model: [[f32; 4]; 4] = Matrix4::new_translation(&Vector3::new(self.position.x as f32 * 16.0,
                                                              self.position.y as f32 * 16.0,
                                                              self.position.z as f32 * 16.0)).into();

        let model_buffer = device
            .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&[(model)]);

        let model_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &model_buffer,
                        range: 0..std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
                    }
                }
            ],
        });

        self.model_bind_group = Some(model_bind_group);
    }

    /*
        This is a complex function that needs to take the xyz of the block position and the direction to create a value

        15, 15, 15  - 1, 0, 0   = 0, 15, 15
        15, 15, 15  - 0, 0, 1   = 15, 15, 0
        0, 0, 0     - -1, 0, 0  = 15, 0, 0
        0, 0, 0     - 0, 0, -1  = 0, 0, 15
        0, 3, 8     - -1, 0, 0  = 15, 3, 8
        15, 3, 8    - 1, 0, 0   = 0, 3, 8
        15, 8, 8    - 1, 0, 0   = 0, 8, 8

        Here's some pseudo code I made up
        If number equals zero
            If any digits are negative
                return 0
            else
                return corresponding number
        else if number equals one
            return 0
        else if number equals negative one
            return 15
     */

    pub fn generate_viewable_map(&self, adjacent_chunks: HashMap<Vector3<i32>, Option<&Chunk>>, chunk_edge_faces: bool) -> [[[ViewableDirection; 16]; 16]; 16] {

        if self.world.is_none() {
            return [[[ViewableDirection(0); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        }

        let mut data = [[[ViewableDirection(0); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        let world = self.world.as_ref().unwrap();

        let directions: [Vector3<i32>; 6] = [
            Vector3::new(1, 0, 0),
            Vector3::new(-1, 0, 0),
            Vector3::new(0, 1, 0),
            Vector3::new(0, -1, 0),
            Vector3::new(0, 0, 1),
            Vector3::new(0, 0, -1)];

        for x in 0..world.len() {
            for z in 0..world[0][0].len() {
                for y in 0..world[0].len() {
                    let mut viewable = calculate_viewable(&self, [x, y, z]);

                    // Temp
                    //viewable = ViewableDirection(0);

                    for direction in directions.iter() {

                        // Calculates if block is bordering on this direction
                        if (direction.x == 1 && x == 15) || (direction.x == -1 && x == 0) ||
                            (direction.y == 1 && y == 15) || (direction.y == -1 && y == 0) ||
                            (direction.z == 1 && z == 15) || (direction.z == -1 && z == 0) {

                            // Make it so we get the block on the other chunk closest to our block
                            let block_pos: Vector3<usize> = Vector3::new(
                                if direction.x == 0 {x} else if direction.x == 1 {0} else { 15 },
                                if direction.y == 0 {y} else if direction.y == 1 {0} else { 15 },
                                if direction.z == 0 {z} else if direction.z == 1 {0} else { 15 },
                            );

                            //println!("{:?} - {:?} = {:?}", (x,y,z), direction, block_pos);

                            // Checks if the block in an adjacent chunk is transparent
                            if adjacent_chunks.get(&direction).unwrap().is_some() {

                                let chunk = adjacent_chunks.get(&direction).unwrap().unwrap();

                                let block = {
                                    let world = chunk.world.as_ref();
                                    if world.is_none() {
                                        None
                                    } else {
                                        let block_id = world.unwrap()
                                            [block_pos.x]
                                            [block_pos.y]
                                            [block_pos.z];

                                        if block_id != 0 {
                                            chunk.blocks.get(block_id as usize - 1)
                                        } else {
                                            None
                                        }
                                    }

                                };

                                // Check if face visible
                                if block.map_or(true, |block| block.transparent) {
                                    viewable.add_flag(ViewableDirectionBitMap::from(direction));
                                }
                            } else if chunk_edge_faces {
                                viewable.add_flag(ViewableDirectionBitMap::from(direction));
                            }
                        }
                    }

                    data[x][y][z] = viewable;
                }
            }
        }

        // Check top faces
        data
    }

    pub fn get_block(&self, pos: Vector3<usize>) -> Option<&Block> {
        let block_id = self.world.as_ref().unwrap()[pos.x][pos.y][pos.z];
        if block_id == 0 {
            self.blocks.get(block_id as usize - 1)
        } else {
            None
        }
    }

    pub fn update_mesh(&mut self, data: ChunkMeshData) {
        self.indices = Some(data.indices);
        self.vertices= Some(data.vertices);
        self.viewable_map = data.viewable;
    }
}