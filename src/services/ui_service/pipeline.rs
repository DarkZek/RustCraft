use crate::services::chunk_service::mesh::UIVertex;
use wgpu::{BindGroupLayout, BlendFactor, BlendOperation, Device, RenderPipeline, ShaderModule};

pub fn generate_render_pipeline(
    device: &Device,
    bind_group_layouts: &[&BindGroupLayout],
) -> RenderPipeline {
    let (vs_module, fs_module) = load_shaders(device);

    let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { bind_group_layouts });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &[UIVertex::desc()],
        layout: &render_pipeline_layout,
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
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        color_states: &[wgpu::ColorStateDescriptor {
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            color_blend: wgpu::BlendDescriptor {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            alpha_blend: wgpu::BlendDescriptor {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
            write_mask: wgpu::ColorWrite::ALL,
        }],
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        depth_stencil_state: None,
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    })
}

pub fn load_shaders(device: &Device) -> (ShaderModule, ShaderModule) {
    let vs_src = include_str!("../../../assets/shaders/ui.vert");
    let fs_src = include_str!("../../../assets/shaders/ui.frag");

    let mut compiler = shaderc::Compiler::new().unwrap();

    let vs_spirv = compiler
        .compile_into_spirv(
            vs_src,
            shaderc::ShaderKind::Vertex,
            "shader.glsl",
            "main",
            None,
        )
        .unwrap();

    let fs_spirv = compiler
        .compile_into_spirv(
            fs_src,
            shaderc::ShaderKind::Fragment,
            "shader.glsl",
            "main",
            None,
        )
        .unwrap();

    let vs_module = device.create_shader_module(vs_spirv.as_binary());
    let fs_module = device.create_shader_module(fs_spirv.as_binary());

    (vs_module, fs_module)
}
