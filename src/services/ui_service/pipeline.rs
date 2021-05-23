use crate::render::shaders::load_shaders;
use crate::render::TEXTURE_FORMAT;
use crate::services::chunk_service::mesh::UIVertex;
use wgpu::{
    BindGroupLayout, BlendComponent, BlendState, Device, MultisampleState, RenderPipeline,
    VertexState,
};

/// Creates the user inferace render pipeline. This includes things like loading shaders.
/// This happens because we have one render pass for the chunks, and a separate for user interfaces. This lets us use 2d vertices for UI as well as have more control over depth and perspective.
pub fn generate_render_pipeline(
    device: &Device,
    bind_group_layouts: &[&BindGroupLayout],
) -> RenderPipeline {
    let (vs_module, fs_module) = load_shaders(
        device,
        (
            include_bytes!("../../../../RustCraft/assets/shaders/ui_text_vert.spv"),
            include_bytes!("../../../../RustCraft/assets/shaders/ui_text_frag.spv"),
        ),
    );

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("UI Render Pipeline"),
        bind_group_layouts,
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
            clamp_depth: false,
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
                format: TEXTURE_FORMAT.get().unwrap().clone(),
                write_mask: wgpu::ColorWrite::ALL,
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
    })
}
