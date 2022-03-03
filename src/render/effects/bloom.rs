use crate::render::effects::EffectPasses;
use crate::render::vertices::UIVertex;
use crate::render::{get_texture_format, VERTICES_COVER_SCREEN};
use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
    CommandEncoder, Device, RenderPassColorAttachment, RenderPipeline, SamplerBindingType,
    SamplerDescriptor, ShaderStages, Texture, TextureSampleType, TextureViewDescriptor,
    TextureViewDimension, VertexState,
};

pub struct BloomPostProcessingEffect {
    pub bloom_render_pipeline: RenderPipeline,
    pub bloom_bind_group_layout: BindGroupLayout,
}

impl BloomPostProcessingEffect {
    pub fn new(device: &Device) -> BloomPostProcessingEffect {
        let bloom_vert_shader = device.create_shader_module(&wgpu::include_spirv!(
            "../../../assets/shaders/gaussian_vert.spv"
        ));

        let bloom_frag_shader = device.create_shader_module(&wgpu::include_spirv!(
            "../../../assets/shaders/gaussian_frag.spv"
        ));

        let bloom_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Gaussian Blur Bind Group Layout"),
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
                    ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
        });

        let bloom_render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Gaussian effect pipeline layout descriptor"),
                bind_group_layouts: &[&bloom_bind_group_layout],
                push_constant_ranges: &[wgpu::PushConstantRange {
                    stages: wgpu::ShaderStages::FRAGMENT,
                    range: 0..4,
                }],
            });

        let bloom_render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Gaussian effect pipeline"),
                layout: Option::from(&bloom_render_pipeline_layout),
                vertex: VertexState {
                    module: &bloom_vert_shader,
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
                    module: &bloom_frag_shader,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: get_texture_format(),
                        write_mask: wgpu::ColorWrites::ALL,
                        blend: None,
                    }],
                }),
                multiview: None,
            });

        BloomPostProcessingEffect {
            bloom_render_pipeline,
            bloom_bind_group_layout,
        }
    }

    pub fn create_bloom_effect(
        &self,
        effect_passes: &mut EffectPasses,
        encoder: &mut CommandEncoder,
        bloom_texture: &Texture,
        frame: &Texture,
    ) {
        let pingpong_buffer = effect_passes.get_buffer();

        let pingpong_buffer_view =
            (*pingpong_buffer).create_view(&TextureViewDescriptor::default());

        let bloom_texture_view = bloom_texture.create_view(&TextureViewDescriptor::default());

        let sampler = effect_passes
            .device
            .create_sampler(&SamplerDescriptor::default());

        let bind_group = effect_passes
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Gaussian Effect Bind Group"),
                layout: &self.bloom_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(&bloom_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&sampler),
                    },
                ],
            });

        let bind_pingpong_group =
            effect_passes
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Gaussian Effect Pingpong Bind Group"),
                    layout: &self.bloom_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: BindingResource::TextureView(&pingpong_buffer_view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: BindingResource::Sampler(&sampler),
                        },
                    ],
                });

        let amount = 20;

        // Amount must be even!
        assert_eq!(amount % 2, 0);

        let mut horizontal = 0;

        // If the input image should be pingpong and output should be the bloom buffer
        let mut input_is_pingpong = false;

        for _ in 0..amount {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Gaussian Blur Render Pass Stage"),
                color_attachments: &[RenderPassColorAttachment {
                    // Output image
                    view: if input_is_pingpong {
                        &bloom_texture_view
                    } else {
                        &pingpong_buffer_view
                    },
                    resolve_target: None,
                    ops: Default::default(),
                }],
                depth_stencil_attachment: None,
            });

            pass.set_pipeline(&self.bloom_render_pipeline);

            // Set variables
            pass.set_bind_group(
                0,
                if input_is_pingpong {
                    &bind_pingpong_group
                } else {
                    &bind_group
                },
                &[],
            );

            pass.set_push_constants(ShaderStages::FRAGMENT, 0, &[horizontal, 0, 0, 0]);

            pass.set_vertex_buffer(0, VERTICES_COVER_SCREEN.get().unwrap().slice(..));

            pass.draw(0..6, 0..1);

            if horizontal == 0 {
                horizontal = 1;
            } else {
                horizontal = 0;
            }

            input_is_pingpong = !input_is_pingpong;
        }

        effect_passes
            .merge
            .clone()
            .merge(effect_passes, encoder, &bloom_texture_view, frame);

        effect_passes.return_buffer(pingpong_buffer);
    }
}
