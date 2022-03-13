use crate::render::{get_device, get_swapchain_format};
use crate::vertex::UIVertex;
use wgpu::{BindGroupLayout, RenderPipeline, VertexState};

/// Generates structures required for rendering default elements in components
pub(crate) fn default_render_pipeline(
    projection_bind_group_layout: &BindGroupLayout,
) -> RenderPipeline {
    let vert_shader =
        get_device().create_shader_module(&wgpu::include_spirv!("../../shaders/default_vert.spv"));

    let frag_shader =
        get_device().create_shader_module(&wgpu::include_spirv!("../../shaders/default_frag.spv"));

    let render_pipeline_layout =
        get_device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("UI Default pipeline layout descriptor"),
            bind_group_layouts: &[&projection_bind_group_layout],
            push_constant_ranges: &[],
        });

    get_device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("UI Default pipeline"),
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
                blend: None,
            }],
        }),
        multiview: None,
    })
}
