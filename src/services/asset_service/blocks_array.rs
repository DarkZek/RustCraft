//
// The purpose of this file is to generate the array of block textures
//

use image::{ImageFormat, DynamicImage};
use std::fs::File;
use std::io::Read;
use wgpu::{Texture, Sampler};
use crate::services::ServicesContext;

pub fn generate_blocks_array(context: &mut ServicesContext) -> (Texture, Sampler) {

    let mut encoder = context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        todo: 0,
    });

    let block_size = 32;

    let diffuse_texture = context.device.create_texture(&wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width: block_size,
            height: block_size,
            depth: 1,
        },
        array_layer_count: 2,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    });

    let mut image = File::open("/home/darkzek/Documents/Projects/AshLearning/target/release/resources/birch_log.png").unwrap();

    let mut data: Vec<u8> = Vec::new();
    if let Err(e) = image.read_to_end(&mut data) {}
    let img = DynamicImage::ImageRgba8(image::load_from_memory_with_format(&data, ImageFormat::Png).unwrap().into_rgba());
    let diffuse_rgba = img.as_rgba8().unwrap();

    let diffuse_buffer = context.device
        .create_buffer_mapped(diffuse_rgba.len(), wgpu::BufferUsage::COPY_SRC)
        .fill_from_slice(&diffuse_rgba);

    encoder.copy_buffer_to_texture(
        wgpu::BufferCopyView {
            buffer: &diffuse_buffer,
            offset: 0,
            row_pitch: 4 * block_size,
            image_height: block_size,
        },
        wgpu::TextureCopyView {
            texture: &diffuse_texture,
            mip_level: 0,
            array_layer: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        wgpu::Extent3d {
            width: block_size,
            height: block_size,
            depth: 1,
        },
    );

    let mut image = File::open("/home/darkzek/Documents/Projects/AshLearning/target/release/resources/andesite.png").unwrap();

    let mut data: Vec<u8> = Vec::new();
    if let Err(e) = image.read_to_end(&mut data) {}

    let img = DynamicImage::ImageRgba8(image::load_from_memory_with_format(&data, ImageFormat::Png).unwrap().into_rgba());
    let diffuse_rgba = img.as_rgba8().unwrap();

    let diffuse_buffer = context.device
        .create_buffer_mapped(diffuse_rgba.len(), wgpu::BufferUsage::COPY_SRC)
        .fill_from_slice(&diffuse_rgba);

    encoder.copy_buffer_to_texture(
        wgpu::BufferCopyView {
            buffer: &diffuse_buffer,
            offset: 0,
            row_pitch: 4 * block_size,
            image_height: block_size,
        },
        wgpu::TextureCopyView {
            texture: &diffuse_texture,
            mip_level: 0,
            array_layer: 1,
            origin: wgpu::Origin3d::ZERO,
        },
        wgpu::Extent3d {
            width: block_size,
            height: block_size,
            depth: 1,
        },
    );

    context.queue.submit(&[
        encoder.finish()
    ]);

    let diffuse_sampler = context.device.create_sampler(&wgpu::SamplerDescriptor {
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

    (diffuse_texture, diffuse_sampler)
}