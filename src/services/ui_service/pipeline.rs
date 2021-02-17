use crate::render::shaders::bytes_to_shader;
use crate::services::chunk_service::mesh::UIVertex;
use wgpu::{
    BindGroupLayout, BlendFactor, BlendOperation, Device, RenderPipeline, ShaderModule,
    VertexStateDescriptor,
};

/// Creates the user inferace render pipeline. This includes things like loading shaders.
/// This happens because we have one render pass for the chunks, and a separate for user interfaces. This lets us use 2d vertices for UI as well as have more control over depth and perspective.
pub fn generate_render_pipeline(
    device: &Device,
    bind_group_layouts: &[&BindGroupLayout],
) -> RenderPipeline {
    let (vs_module, fs_module) = load_shaders(device);

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts,
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Main render pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fs_module,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::None,
            clamp_depth: false,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        color_states: &[wgpu::ColorStateDescriptor {
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            color_blend: wgpu::BlendDescriptor {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha_blend: wgpu::BlendDescriptor {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::Zero,
                operation: wgpu::BlendOperation::Add,
            },
            write_mask: wgpu::ColorWrite::ALL,
        }],
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        depth_stencil_state: None,
        vertex_state: VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[UIVertex::desc()],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    })
}

pub fn load_shaders(device: &Device) -> (ShaderModule, ShaderModule) {
    let vs_src = include_bytes!("../../../assets/shaders/ui_vert.spv");
    let fs_src = include_bytes!("../../../assets/shaders/ui_frag.spv");

    let vs_module = device.create_shader_module(wgpu::ShaderModuleSource::SpirV(
        std::borrow::Cow::Borrowed(bytes_to_shader(vs_src).as_slice()),
    ));
    let fs_module = device.create_shader_module(wgpu::ShaderModuleSource::SpirV(
        std::borrow::Cow::Borrowed(bytes_to_shader(fs_src).as_slice()),
    ));

    (vs_module, fs_module)
}
