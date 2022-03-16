use crate::render::device::get_device;
use crate::render::effects::EffectPasses;
use crate::render::{get_swapchain_size, VERTICES_COVER_SCREEN};
use nalgebra::Vector3;
use rand::Rng;
use rc_ui::vertex::UIVertex;

use std::mem;

use crate::render::effects::buffer_pool::TextureBufferPool;
use wgpu::util::DeviceExt;
use wgpu::{
    AddressMode, BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingResource, BindingType, Buffer, BufferBindingType, BufferUsages, CommandEncoder,
    Extent3d, Origin3d, Queue, RenderPassColorAttachment, RenderPipeline, Sampler,
    SamplerBindingType, SamplerDescriptor, ShaderStages, Texture, TextureAspect, TextureFormat,
    TextureSampleType, TextureUsages, TextureViewDescriptor, TextureViewDimension, VertexState,
};

pub struct SSAOEffect {
    ssao_render_pipeline: RenderPipeline,
    rotations_texture: Texture,
    ssao_bind_group_layout: BindGroupLayout,
    kernel_sample: Buffer,
    sampler: Sampler,
    pub(crate) ssao_color: Texture,
}

impl SSAOEffect {
    pub fn new(queue: &mut Queue) -> SSAOEffect {
        let encoder = get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("SSAO Setup Command Encoder"),
        });

        queue.submit([encoder.finish()]);

        let ssao_vert_shader = get_device()
            .create_shader_module(&wgpu::include_spirv!("../../../shaders/ssao_vert.spv"));

        let ssao_frag_shader = get_device()
            .create_shader_module(&wgpu::include_spirv!("../../../shaders/ssao_frag.spv"));

        let ssao_bind_group_layout =
            get_device().create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("SSAO Bind Group Layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: false },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: false },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: false },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 3,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 4,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        let rotations_texture = Self::create_rotations_texture(queue);

        let ssao_render_pipeline_layout =
            get_device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("SSAO effect pipeline layout descriptor"),
                bind_group_layouts: &[
                    &ssao_bind_group_layout,
                    // Uniforms containing projection & view matrix
                    &get_device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        entries: &[
                            wgpu::BindGroupLayoutEntry {
                                binding: 0,
                                visibility: wgpu::ShaderStages::FRAGMENT,
                                ty: wgpu::BindingType::Buffer {
                                    ty: BufferBindingType::Uniform,
                                    min_binding_size: None,
                                    has_dynamic_offset: false,
                                },
                                count: None,
                            },
                            wgpu::BindGroupLayoutEntry {
                                binding: 1,
                                visibility: wgpu::ShaderStages::FRAGMENT,
                                ty: wgpu::BindingType::Buffer {
                                    ty: BufferBindingType::Uniform,
                                    min_binding_size: None,
                                    has_dynamic_offset: false,
                                },
                                count: None,
                            },
                        ],
                        label: Some("Unknown uniform buffer bind group layout"),
                    }),
                ],
                push_constant_ranges: &[],
            });

        let ssao_render_pipeline =
            get_device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("SSAO effect pipeline"),
                layout: Option::from(&ssao_render_pipeline_layout),
                vertex: VertexState {
                    module: &ssao_vert_shader,
                    entry_point: "main",
                    buffers: &[UIVertex::desc()],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &ssao_frag_shader,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: TextureFormat::R8Unorm,
                        write_mask: wgpu::ColorWrites::ALL,
                        blend: None,
                    }],
                }),
                multiview: None,
            });

        let kernel_sample = Self::get_kernel_sample();

        let sampler = get_device().create_sampler(&SamplerDescriptor {
            /// How to deal with out of bounds accesses in the u (i.e. x) direction
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::ClampToEdge,
            ..SamplerDescriptor::default()
        });

        let ssao_color = Self::get_ssao_texture();

        SSAOEffect {
            ssao_render_pipeline,
            rotations_texture,
            ssao_bind_group_layout,
            kernel_sample,
            sampler,
            ssao_color,
        }
    }

    pub fn resize(&mut self) {
        self.ssao_color = Self::get_ssao_texture();
    }

    pub fn get_ssao_texture() -> Texture {
        get_device().create_texture(&wgpu::TextureDescriptor {
            label: Some("SSAO Color texture"),
            size: get_swapchain_size(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_SRC,
        })
    }

    pub fn get_kernel_sample() -> Buffer {
        get_device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("SSAO Kernel Hemisphere Sample Buffer"),
            contents: &bytemuck::cast_slice(&Self::get_hemisphere()),
            usage: BufferUsages::UNIFORM,
        })
    }

    pub fn get_hemisphere() -> [[f32; 4]; 64] {
        let mut rand = rand::thread_rng();
        let mut ssao_kernel = [[0.0; 4]; 64];

        for i in 0..64 {
            let mut sample = Vector3::new(
                rand.gen_range(0.0, 1.0) * 2.0 - 1.0,
                rand.gen_range(0.0, 1.0) * 2.0 - 1.0,
                rand.gen_range(0.0, 1.0) as f32,
            );

            sample = sample.normalize();
            sample *= rand.gen_range(0.0, 1.0);

            let mut scale = i as f32 / 64.0;

            // scale samples s.t. they're more aligned to center of kernel
            scale = lerp(0.1, 1.0, scale * scale);
            sample *= scale;

            ssao_kernel[i] = [sample.x, sample.y, sample.z, 0.0];
        }

        ssao_kernel
    }

    pub fn create_rotations_texture(queue: &mut Queue) -> Texture {
        let size = 4;
        let rgb = Self::get_rotations(size);

        let rotation_texture = get_device().create_texture(&wgpu::TextureDescriptor {
            label: Some("SSAO Rotations Texture"),
            size: wgpu::Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &rotation_texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            bytemuck::cast_slice(rgb.as_slice()),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * 4 * size),
                rows_per_image: std::num::NonZeroU32::new(size),
            },
            Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: 1,
            },
        );

        rotation_texture
    }

    pub fn get_rotations(size: u32) -> Vec<[f32; 4]> {
        let mut rand = rand::thread_rng();
        let mut ssao_noise = Vec::new();

        for _i in 0..(size * size) {
            let noise = [
                rand.gen_range(0.0, 1.0) * 2.0 - 1.0,
                rand.gen_range(0.0, 1.0) * 2.0 - 1.0,
                0.0,
                1.0 as f32,
            ];

            ssao_noise.push(noise);
        }

        ssao_noise
    }

    pub fn apply_ssao(
        &self,
        effect_passes: &EffectPasses,
        encoder: &mut CommandEncoder,
        buffer_pool: &mut TextureBufferPool,
        projection_bind_group: &BindGroup,
        dest: &Texture,
    ) {
        let ssao_data_view = self
            .ssao_color
            .create_view(&TextureViewDescriptor::default());

        let bind_group = get_device().create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("SSAO Effect Bind Group"),
            layout: &self.ssao_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(
                        &effect_passes
                            .position_map
                            .create_view(&TextureViewDescriptor::default()),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(
                        &effect_passes
                            .normal_map
                            .create_view(&TextureViewDescriptor::default()),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::TextureView(
                        &self
                            .rotations_texture
                            .create_view(&TextureViewDescriptor::default()),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: BindingResource::Buffer(
                        self.kernel_sample.as_entire_buffer_binding(),
                    ),
                },
            ],
        });

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("SSAO Render Pass"),
            color_attachments: &[RenderPassColorAttachment {
                // Output image
                view: &ssao_data_view,
                resolve_target: None,
                ops: Default::default(),
            }],
            depth_stencil_attachment: None,
        });

        pass.set_pipeline(&self.ssao_render_pipeline);

        // Set variables
        pass.set_bind_group(0, &bind_group, &[]);
        pass.set_bind_group(1, projection_bind_group, &[]);

        pass.set_vertex_buffer(0, VERTICES_COVER_SCREEN.get().unwrap().slice(..));

        pass.draw(0..6, 0..1);

        mem::drop(pass);

        effect_passes.effect_multiply.multiply(
            effect_passes,
            encoder,
            buffer_pool,
            &self
                .ssao_color
                .create_view(&TextureViewDescriptor::default()),
            dest,
        );
    }
}

fn lerp(a: f32, b: f32, f: f32) -> f32 {
    a + f * (b - a)
}
