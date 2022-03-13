use crate::render::{get_device, get_swapchain_format};
use crate::vertex::UIVertex;
use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendState,
    RenderPipeline, SamplerBindingType, ShaderStages, TextureSampleType, TextureViewDimension,
    VertexState,
};

/// Generates structures required for Combination functionality of the UI
pub(crate) fn combine_render_pipeline(
    projection_bind_group_layout: &BindGroupLayout,
) -> (RenderPipeline, BindGroupLayout) {
    let vert_shader =
        get_device().create_shader_module(&wgpu::include_spirv!("../../shaders/combine_vert.spv"));

    let frag_shader =
        get_device().create_shader_module(&wgpu::include_spirv!("../../shaders/combine_frag.spv"));

    let bind_group_layout = get_device().create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("UI Combine Bind Group Layout"),
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

    let render_pipeline_layout =
        get_device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("UI Combine pipeline layout descriptor"),
            bind_group_layouts: &[&projection_bind_group_layout, &bind_group_layout],
            push_constant_ranges: &[],
        });

    let render_pipeline = get_device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("UI Combine pipeline"),
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
                format: *get_swapchain_format(),
                write_mask: wgpu::ColorWrites::ALL,
                blend: Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING),
            }],
        }),
        multiview: None,
    });

    (render_pipeline, bind_group_layout)
}
