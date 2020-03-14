use crate::block::Block;
use image::{ImageFormat, GenericImageView};
use wgpu::{Sampler, Queue, Device, Texture};

const TEXTURE_DIMENSIONS: u32 = 16;

/// Loads textures and updates blocks list with dynamic texture ID
pub fn load_textures(blocks: &mut Vec<Block>, queue: &mut Queue, device: &Device) -> (Sampler, Texture) {
    let mut name_to_id = Vec::new();

    // Compile texture names into a list and update blocks
    for block in blocks.iter_mut() {
        let mut texture_id: [u32; 6] = [0; 6];

        let i = 0;

        for texture_name in &block.raw_texture_names {
            if name_to_id.contains(texture_name) {
                texture_id[i] = index_of(&name_to_id, texture_name).unwrap_or(0);
            } else {
                texture_id[i] = name_to_id.len() as u32;
                name_to_id.push(texture_name);
            }
        }

        block.texture_ids = texture_id;
    }

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

    // Create buffers
    let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width: TEXTURE_DIMENSIONS,
            height: TEXTURE_DIMENSIONS,
            depth: 1,
        },
        array_layer_count: blocks.len() as u32,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        todo: 0,
    });

    // Load textures
    for (i, texture_name) in name_to_id.iter().enumerate() {

        let path = format!("/home/darkzek/Documents/Projects/AshLearning/assets/textures/blocks/{}.png", texture_name);

        let diffuse_image = image::open(&path).unwrap();
        let diffuse_rgba = diffuse_image.as_rgba8().unwrap();

        let dimensions = diffuse_image.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth: 1,
        };

        let diffuse_buffer = device
            .create_buffer_mapped(diffuse_rgba.len(), wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&diffuse_rgba);

        // Add it to buffer
        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &diffuse_buffer,
                offset: 0,
                row_pitch: 4 * size.width, // the width of the texture in bytes
                image_height: size.height,
            },
            wgpu::TextureCopyView {
                texture: &diffuse_texture,
                mip_level: 0,
                array_layer: i as u32,
                origin: wgpu::Origin3d::ZERO,
            },
            size,
        );
    }

    queue.submit(&[encoder.finish()]);

    (diffuse_sampler, diffuse_texture)
}

fn index_of(array: &Vec<&str>, search: &str) -> Option<u32> {
    for (i, value) in array.iter().enumerate() {
        if value == &search {
            return Some(i as u32);
        }
    }

    None
}