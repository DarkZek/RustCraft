use crate::render::{get_swapchain_size, get_texture_format, VERTICES_COVER_SCREEN};

use crate::render::device::get_device;
use crate::render::effects::EffectPasses;
use rc_ui::vertex::UIVertex;
use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
    CommandEncoder, LoadOp, Operations, RenderPassColorAttachment, RenderPipeline, Sampler,
    SamplerBindingType, SamplerDescriptor, ShaderStages, Texture, TextureSampleType, TextureView,
    TextureViewDescriptor, TextureViewDimension, VertexState,
};

// Merges two textures
pub struct MergePostProcessingEffect {
    pub render_pipeline: RenderPipeline,
    pub bind_group_layout: BindGroupLayout,
    pub sampler: Sampler,
}

impl MergePostProcessingEffect {
    pub fn new() -> MergePostProcessingEffect {
        let vert_shader = get_device()
            .create_shader_module(&wgpu::include_spirv!("../../../shaders/addition_vert.spv"));

        let frag_shader = get_device()
            .create_shader_module(&wgpu::include_spirv!("../../../shaders/addition_frag.spv"));

        let bind_group_layout = get_device().create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Merge Bind Group Layout"),
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
                    ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
        });

        let render_pipeline_layout =
            get_device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Merge effect pipeline layout descriptor"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline =
            get_device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Merge effect pipeline"),
                layout: Option::from(&render_pipeline_layout),
                vertex: VertexState {
                    module: &vert_shader,
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
                    module: &frag_shader,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: *get_texture_format(),
                        write_mask: wgpu::ColorWrites::ALL,
                        blend: None,
                    }],
                }),
                multiview: None,
            });

        let sampler = get_device().create_sampler(&SamplerDescriptor::default());

        MergePostProcessingEffect {
            render_pipeline,
            bind_group_layout,
            sampler,
        }
    }

    pub fn merge(
        &self,
        effect_passes: &mut EffectPasses,
        encoder: &mut CommandEncoder,
        src: &TextureView,
        dest: &Texture,
    ) {
        let temp_image = effect_passes.get_buffer();

        let temp_image_view = temp_image.create_view(&TextureViewDescriptor::default());

        // Copy image to temp buffer
        encoder.copy_texture_to_texture(
            dest.as_image_copy(),
            temp_image.as_image_copy(),
            get_swapchain_size(),
        );

        let bind_group = get_device().create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Merge Effect Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&src),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&temp_image_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(&self.sampler),
                },
            ],
        });

        let destination_view = dest.create_view(&TextureViewDescriptor::default());

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Merge Render Pass Stage"),
            color_attachments: &[RenderPassColorAttachment {
                view: &destination_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        pass.set_pipeline(&self.render_pipeline);

        // Set variables
        pass.set_bind_group(0, &bind_group, &[]);

        pass.set_vertex_buffer(0, VERTICES_COVER_SCREEN.get().unwrap().slice(..));

        pass.draw(0..6, 0..1);

        effect_passes.return_buffer(temp_image);
    }
}
