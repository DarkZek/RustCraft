use wgpu::{Device, Texture, Sampler, TextureComponentType, BufferUsage, TextureDataLayout, Queue, BindGroup, BindGroupLayout, CompareFunction};
use image::ImageFormat;

pub fn load_splash(device: &Device, queue: &mut Queue) -> (Texture, Sampler, BindGroupLayout, BindGroup) {
    let splash_image = include_bytes!("../../../assets/splash.png");
    let splash_image = image::load_from_memory_with_format(splash_image, ImageFormat::Png).unwrap();

    let diffuse_rgba = splash_image.as_rgba8().unwrap();
    let dimensions = diffuse_rgba.dimensions();

    let size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth: 1,
    };

    let diffuse_buffer = device.create_buffer_with_data(&diffuse_rgba, BufferUsage::COPY_SRC);

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    });

    // Add it to buffer
    encoder.copy_buffer_to_texture(
        wgpu::BufferCopyView {
            buffer: &diffuse_buffer,
            layout: TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * size.width,
                rows_per_image: size.height,
            },
        },
        wgpu::TextureCopyView {
            texture: &diffuse_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        size,
    );

    queue.submit(Some(encoder.finish()));

    let diffuse_sampler_descriptor = wgpu::SamplerDescriptor {
        label: None,
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        lod_min_clamp: -100.0,
        lod_max_clamp: 100.0,
        compare: Some(CompareFunction::Always),
        anisotropy_clamp: None,
        _non_exhaustive: Default::default(),
    };

    let diffuse_sampler = device.create_sampler(&diffuse_sampler_descriptor);

    let bindings = load_splash_image_bindings(device, &diffuse_texture, &diffuse_sampler);

    (diffuse_texture, diffuse_sampler, bindings.0, bindings.1)
}

pub fn load_splash_image_bindings(device: &Device,
                                  diffuse_texture: &Texture,
                                  diffuse_sampler: &Sampler) -> (BindGroupLayout, BindGroup) {
    let diffuse_texture_view = diffuse_texture.create_default_view();

    let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        dimension: wgpu::TextureViewDimension::D2Array,
                        component_type: TextureComponentType::Float,
                    },
                    count: None,
                    _non_exhaustive: Default::default(),
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: true },
                    count: None,
                    _non_exhaustive: Default::default(),
                },
            ],
            label: None,
        });

    let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        bindings: &[
            wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
            },
            wgpu::Binding {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(diffuse_sampler),
            },
        ],
        label: None,
    });

    (texture_bind_group_layout, texture_bind_group)
}

// pub fn load_view_matrix(device: &Device) {
//
//     let uniform_buffer = device.create_buffer_with_data(
//         bytemuck::cast_slice(&self.view_proj),
//         wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::COPY_SRC,
//     );
//
//     let uniform_bind_group_layout =
//         device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//             bindings: &[wgpu::BindGroupLayoutEntry {
//                 binding: 0,
//                 visibility: wgpu::ShaderStage::VERTEX,
//                 ty: wgpu::BindingType::UniformBuffer {
//                     dynamic: false,
//                     min_binding_size: None,
//                 },
//                 count: None,
//                 _non_exhaustive: Default::default(),
//             }],
//             label: None,
//         });
//
//     let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
//         layout: &uniform_bind_group_layout,
//         bindings: &[wgpu::Binding {
//             binding: 0,
//             resource: wgpu::BindingResource::Buffer(
//                 uniform_buffer.slice(0..std::mem::size_of_val(&self) as wgpu::BufferAddress),
//             ),
//         }],
//         label: None,
//     });
//
//     (
//         uniform_buffer,
//         uniform_bind_group_layout,
//         uniform_bind_group,
//     )
// }