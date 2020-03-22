use crate::block::Block;
use wgpu::{Sampler, Queue, Device, Texture};
use crate::render::texture::atlas::{generate_texture_atlas, TextureAtlasIndex};

/// Loads textures and updates blocks list with dynamic texture ID
pub fn load_textures(blocks: &mut Vec<Block>, queue: &mut Queue, device: &Device) -> (Sampler, Texture, Vec<TextureAtlasIndex>) {
    let mut name_to_id = Vec::new();

    // Compile texture names into a list and update blocks
    for block in blocks.iter_mut() {
        let mut texture_id: [u32; 6] = [666; 6];

        for (i, texture_name) in block.raw_texture_names.iter().enumerate() {
            if name_to_id.contains(texture_name) {
                texture_id[i] = index_of(&name_to_id, texture_name).unwrap_or(0);
            } else {
                texture_id[i] = name_to_id.len() as u32;
                name_to_id.push(texture_name);
            }
        }

        block.texture_ids = texture_id;
    }

    println!("Loaded textures: {:?}", name_to_id);

    // Create sampler
    let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        lod_min_clamp: -100.0,
        lod_max_clamp: 100.0,
        compare_function: wgpu::CompareFunction::Always,
    });

    // Load textures
    let (diffuse_texture, atlas) = generate_texture_atlas(name_to_id, "/home/darkzek/Documents/Projects/AshLearning/assets/textures/blocks/", &device, queue);

    // Apply atlas lookups
    for block in blocks {
        for (i, atlas_id) in block.texture_ids.iter().enumerate() {
            let index = atlas.get(atlas_id.clone() as usize).unwrap();
            block.texture_atlas_lookups[i] = index.clone();
        }
    }

    (diffuse_sampler, diffuse_texture, atlas)
}

fn index_of(array: &Vec<&str>, search: &str) -> Option<u32> {
    for (i, value) in array.iter().enumerate() {
        if value == &search {
            return Some(i as u32);
        }
    }

    None
}