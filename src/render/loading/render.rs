use crate::render::loading::LoadingScreen;
use crate::render::shaders::load_shaders;
use crate::services::chunk_service::mesh::UIVertex;
use nalgebra::{Matrix4, Orthographic3};
use std::borrow::Cow;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupLayout, BlendFactor, BlendOperation, Buffer, Device, RenderPipeline,
    ShaderModule, VertexStateDescriptor,
};
use winit::dpi::PhysicalSize;

impl LoadingScreen {
    pub(crate) fn generate_loading_render_pipeline(
        device: &Device,
        bind_group_layouts: &[&BindGroupLayout],
    ) -> RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts,
                push_constant_ranges: &[],
            });

        let (vs_module, fs_module) = load_shaders(
            &device,
            (
                include_bytes!("../../../assets/shaders/loading_vert.spv"),
                include_bytes!("../../../assets/shaders/loading_frag.spv"),
            ),
        );

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Option::from(&render_pipeline_layout),
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
            vertex_state: VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[UIVertex::desc()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        })
    }

    pub fn setup_ui_projection_matrix(
        size: PhysicalSize<u32>,
        device: &Device,
    ) -> (Buffer, BindGroup, BindGroupLayout) {
        let ratio = size.width as f32 / size.height as f32;

        let projection = Orthographic3::new(-ratio, ratio, -1.0, 1.0, 0.1, 10.0);

        let matrix_binding_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer {
                    dynamic: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        };

        let matrix: Matrix4<f32> = projection.into();

        let matrix_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: &bytemuck::cast_slice(matrix.as_slice()),
            usage: wgpu::BufferUsage::UNIFORM
                | wgpu::BufferUsage::COPY_DST
                | wgpu::BufferUsage::COPY_SRC,
        });

        let matrix_bind_group_layout =
            device.create_bind_group_layout(&matrix_binding_layout_descriptor);

        let matrix_bind_group_descriptor = wgpu::BindGroupDescriptor {
            layout: &matrix_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    matrix_buffer.slice(0..std::mem::size_of_val(&matrix) as wgpu::BufferAddress),
                ),
            }],
            label: None,
        };

        let matrix_bind_group = device.create_bind_group(&matrix_bind_group_descriptor);

        (matrix_buffer, matrix_bind_group, matrix_bind_group_layout)
    }
}
