use crate::render::device::get_device;
use crate::render::get_texture_format;
use crate::render::vertices::UIVertex;
use wgpu::{
    BindGroupLayout, BlendComponent, BlendState, MultisampleState, RenderPipeline,
    VertexState,
};

/// Creates the user inferace render pipeline. This includes things like loading shaders.
/// This happens because we have one render pass for the chunks, and a separate for user interfaces. This lets us use 2d vertices for UI as well as have more control over depth and perspective.
pub fn generate_render_pipeline(bind_group_layouts: &[&BindGroupLayout]) -> RenderPipeline {
    let vs_module = get_device().create_shader_module(&wgpu::include_spirv!(
        "../../../assets/shaders/ui_text_vert.spv"
    ));
    let fs_module = get_device().create_shader_module(&wgpu::include_spirv!(
        "../../../assets/shaders/ui_text_frag.spv"
    ));

    let render_pipeline_layout =
        get_device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("UI Render Pipeline"),
            bind_group_layouts,
            push_constant_ranges: &[],
        });

    get_device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Main UI render pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: VertexState {
            module: &vs_module,
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
        multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(wgpu::FragmentState {
            module: &fs_module,
            entry_point: "main",
            targets: &[wgpu::ColorTargetState {
                format: get_texture_format(),
                write_mask: wgpu::ColorWrites::ALL,
                blend: Some(BlendState {
                    color: BlendComponent {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: BlendComponent {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::Zero,
                        operation: wgpu::BlendOperation::Add,
                    },
                }),
            }],
        }),
        multiview: None,
    })
}
