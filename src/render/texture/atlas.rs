use image::{GenericImageView, DynamicImage, Rgba};
use std::time::SystemTime;
use wgpu::{Device, Queue, Texture};

pub type TextureAtlasIndex = ([f32; 2], [f32; 2]);

/// Generate a a new texture atlas from a list of textures and a resources directory
pub fn generate_texture_atlas(textures: Vec<&str>, resources: &str, device: &Device, queue: &mut Queue) -> (Texture, Vec<TextureAtlasIndex>) {

    // Leave this unoptimised so that I can easily optimise it later and claim I just made it 20% faster

    let start_time = SystemTime::now();

    let texture_size = 16;
    let atlas_width = 4;
    let atlas_height = (textures.len() as f32 / atlas_width as f32).ceil() as u32;

    //Create buffer
    let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width: atlas_width * texture_size,
            height: atlas_height * texture_size,
            depth: 1,
        },
        array_layer_count: 1,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    });

    let mut atlas = image::ImageBuffer::new(texture_size * atlas_width, texture_size * atlas_height);
    let mut images = Vec::new();

    // Load textures
    for (_i, texture_name) in textures.iter().enumerate() {
        let path = format!("{}{}.png", resources, texture_name);

        match image::open(&path) {
            Ok(img) => {
                images.push(Some(img));
            }
            Err(_e) => {
                println!("Missing texture: {}", path);
                images.push(None);
            }
        }

    }

    // Combine them into a single map
    for (x, y, pixel) in atlas.enumerate_pixels_mut() {
        let texture_number = (x as f32 / texture_size as f32).floor() + ((y as f32 / texture_size as f32).floor() * atlas_width as f32);
        match images.get(texture_number as usize) {
            Some(image) => {
                let tex_x = x % texture_size;
                let tex_y = y % texture_size;

                match image {
                    Some(img) => {
                        *pixel = img.get_pixel(tex_x, tex_y);
                    }
                    None => {
                        *pixel = invalid_texture(tex_x, tex_y, texture_size);
                    }
                }
            }
            None => {
                *pixel = image::Rgba([255, 255, 0, 1]);
            }
        }
    }

    //atlas.save("/home/darkzek/Documents/atlas.png");

    let atlas_img = DynamicImage::ImageRgba8(atlas);
    let diffuse_rgba = atlas_img.as_rgba8().unwrap();

    let dimensions = diffuse_rgba.dimensions();

    let size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth: 1,
    };

    let diffuse_buffer = device
        .create_buffer_mapped(diffuse_rgba.len(), wgpu::BufferUsage::COPY_SRC)
        .fill_from_slice(&diffuse_rgba);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        todo: 0,
    });

    // Add it to buffer
    encoder.copy_buffer_to_texture(
        wgpu::BufferCopyView {
            buffer: &diffuse_buffer,
            offset: 0,
            row_pitch: 4 * size.width,
            image_height: size.height,
        },
        wgpu::TextureCopyView {
            texture: &diffuse_texture,
            mip_level: 0,
            array_layer: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        size,
    );

    queue.submit(&[encoder.finish()]);

    let mut atlas_lookups: Vec<TextureAtlasIndex> = Vec::new();

    // Calculate locations of textures
    for (i, _texture_name) in textures.iter().enumerate() {

        let texture_x = i as f32 % atlas_width as f32;
        let texture_y = (i as f32 / atlas_width as f32).floor();

        // Calculate the starting point
        let start_x = (1.0 / atlas_width as f32) * texture_x;
        let start_y = (1.0 / atlas_height as f32) * texture_y;

        let end_x = (1.0 / atlas_width as f32) * (texture_x + 1.0);
        let end_y = (1.0 / atlas_height as f32) * (texture_y + 1.0);

        atlas_lookups.push(([start_x, start_y], [end_x, end_y]));
    }

    println!("Creating atlas map took: {}ms", start_time.elapsed().unwrap().as_millis());

    (diffuse_texture, atlas_lookups)
}

fn invalid_texture(x: u32, y: u32, texture_size: u32) -> Rgba<u8> {
    let relative_x = ((x as f32 + 1.0) / (texture_size as f32 / 2.0)).ceil();
    let relative_y = ((y as f32 + 1.0) / (texture_size as f32 / 2.0)).ceil();
    let purple = (relative_x + relative_y) % 2.0 == 0.0;
    if purple { image::Rgba([255, 0, 255, 255]) } else { image::Rgba([0, 0, 0, 255]) }
}