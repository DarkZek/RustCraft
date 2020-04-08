use noise::{NoiseFn, Perlin, Seedable};
use crate::block::Block;
use crate::world::CHUNK_SIZE;
use crate::world::chunk::Chunk;
use wgpu::{Device, BindGroupLayout};

pub struct World {
    pub seed: u32,
    pub chunks: Vec<Chunk>,
    pub model_bind_group_layout: BindGroupLayout,
    pub render_distance: u32
}

impl World {

    pub fn new(device: &Device, seed: u32, render_distance: u32) -> World {

        let model_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: true
                    },
                }
            ]
        });

        World {
            seed,
            chunks: Vec::new(),
            model_bind_group_layout,
            render_distance
        }
    }

    pub fn generate_chunk(&mut self, chunk_x: i32, chunk_z: i32, blocks: &Vec<Block>, device: &Device) -> usize {
        let scale = 1.0 / CHUNK_SIZE as f64;

        let noise_map = Perlin::new();
        noise_map.set_seed(self.seed);

        let mut world = [[[0 as u32; CHUNK_SIZE]; 256]; CHUNK_SIZE];
        let blocks: Vec<Block> = (*blocks).to_vec();

        for x in 0..world.len() {
            for z in 0..world[0][0].len() {
                let height_map = noise_map.get([(x as f64 * scale) + chunk_x as f64, (z as f64 * scale) + chunk_z as f64]);
                let height = (height_map * 20.0).round() as i32;

                for y in 0..(height + 50) {
                    world[x][y as usize][z] = 1;
                }

                //Dirt & grass
                world[x][(height + 50) as usize][z] = 2;
                world[x][(height + 51) as usize][z] = 2;
                world[x][(height + 52) as usize][z] = 3;
            }
        }

        let mut chunk = Chunk {
            world,
            blocks,
            vertices: None,
            indices: None,
            vertices_buffer: None,
            indices_buffer: None,
            indices_buffer_len: 0,
            model_bind_group: None,
            x: chunk_x,
            z: chunk_z,
        };

        chunk.generate_mesh();
        chunk.create_buffers(device, &self.model_bind_group_layout);

        self.chunks.push(chunk);
        self.chunks.len()
    }
}