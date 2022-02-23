use crate::render::get_texture_format;
use image::ImageFormat;
use std::convert::TryFrom;
use std::num::NonZeroU32;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupLayout, BufferUsages, Device, ImageDataLayout, Queue, Sampler,
    SamplerBindingType, Texture, TextureAspect, TextureSampleType, TextureViewDescriptor,
    TextureViewDimension,
};

pub fn load_splash(
    device: &Device,
    queue: &mut Queue,
) -> (Texture, Sampler, BindGroupLayout, BindGroup) {
    let splash_image = include_bytes!("../../../../RustCraft/assets/splash.png");
    let splash_image = image::load_from_memory_with_format(splash_image, ImageFormat::Png).unwrap();

    let diffuse_rgba = splash_image.as_rgba8().unwrap();
    let dimensions = diffuse_rgba.dimensions();

    let size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };

    let diffuse_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Loading splash screen image buffer"),
        contents: &diffuse_rgba,
        usage: BufferUsages::COPY_SRC,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Loading splash screen command encoder"),
    });

    let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Loading splash screen texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: get_texture_format(),
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    });

    // Add it to buffer
    encoder.copy_buffer_to_texture(
        wgpu::ImageCopyBuffer {
            buffer: &diffuse_buffer,
            layout: ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(NonZeroU32::try_from(4 * size.width).unwrap()),
                rows_per_image: Some(NonZeroU32::try_from(size.height).unwrap()),
            },
        },
        wgpu::ImageCopyTexture {
            texture: &diffuse_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: TextureAspect::All,
        },
        size,
    );

    queue.submit(Some(encoder.finish()));

    let diffuse_sampler_descriptor = wgpu::SamplerDescriptor {
        label: Some("Loading splash screen sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        lod_min_clamp: -100.0,
        lod_max_clamp: 100.0,
        compare: None,
        anisotropy_clamp: None,
        border_color: None,
    };

    let diffuse_sampler = device.create_sampler(&diffuse_sampler_descriptor);

    let bindings = load_splash_image_bindings(device, &diffuse_texture, &diffuse_sampler);

    (diffuse_texture, diffuse_sampler, bindings.0, bindings.1)
}

pub fn load_splash_image_bindings(
    device: &Device,
    diffuse_texture: &Texture,
    diffuse_sampler: &Sampler,
) -> (BindGroupLayout, BindGroup) {
    let diffuse_texture_view = diffuse_texture.create_view(&TextureViewDescriptor {
        label: Some("Loading splash screen texture descriptor"),
        format: Some(get_texture_format()),
        dimension: Some(TextureViewDimension::D2),
        aspect: TextureAspect::All,
        base_mip_level: 0,
        mip_level_count: None,
        base_array_layer: 0,
        array_layer_count: None,
    });

    let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
            label: Some("Loading splash screen bind group layout"),
        });

    let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(diffuse_sampler),
            },
        ],
        label: Some("Loading splash screen bind group"),
    });

    (texture_bind_group_layout, texture_bind_group)
}
