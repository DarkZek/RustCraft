use wgpu::{Device, BindGroupLayout, RenderPipeline, ShaderModule};
use crate::services::chunk_service::mesh::UIVertex;

pub fn generate_render_pipeline(device: &Device, bind_group_layouts: &[&BindGroupLayout]) -> RenderPipeline {

    let (vs_module, fs_module) = load_shaders(device);

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &[
            UIVertex::desc(),
        ],
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
        color_states: &[
            wgpu::ColorStateDescriptor {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            },
        ],
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        depth_stencil_state: None,
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false
    })
}

pub fn load_shaders(device: &Device) -> (ShaderModule, ShaderModule){
    let vs_src = include_str!("../../render/shaders/ui.vert");
    let fs_src = include_str!("../../render/shaders/ui.frag");

    let vs_spirv = glsl_to_spirv::compile(vs_src, glsl_to_spirv::ShaderType::Vertex).unwrap();
    let fs_spirv = glsl_to_spirv::compile(fs_src, glsl_to_spirv::ShaderType::Fragment).unwrap();

    let vs_data = wgpu::read_spirv(vs_spirv).unwrap();
    let fs_data = wgpu::read_spirv(fs_spirv).unwrap();

    let vs_module = device.create_shader_module(&vs_data);
    let fs_module = device.create_shader_module(&fs_data);

    (vs_module, fs_module)
}