use crate::render::loading::LoadingScreen;
use crate::render::shaders::load_shaders;
use crate::render::TEXTURE_FORMAT;
use crate::services::chunk_service::mesh::UIVertex;
use nalgebra::{Matrix4, Orthographic3};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupLayout, BlendComponent, BlendState, Buffer, BufferBinding,
    BufferBindingType, Device, RenderPipeline, VertexState,
};
use winit::dpi::PhysicalSize;

impl LoadingScreen {
    pub fn generate_loading_render_pipeline(
        device: &Device,
        bind_group_layouts: &[&BindGroupLayout],
    ) -> RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Loading screen pipeline layout descriptor"),
                bind_group_layouts,
                push_constant_ranges: &[],
            });

        let (vs_module, fs_module) = load_shaders(
            &device,
            (
                include_bytes!("../../../../RustCraft/assets/shaders/loading_vert.spv"),
                include_bytes!("../../../../RustCraft/assets/shaders/loading_frag.spv"),
            ),
        );

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Loading render pipeline"),
            layout: Option::from(&render_pipeline_layout),
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
            multisample: wgpu::MultisampleState {
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
                ty: wgpu::BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            }],
            label: Some("Loading screen projection matrix bind group"),
        };

        let matrix: Matrix4<f32> = projection.into();

        let matrix_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Loading screen projection matrix buffer"),
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
                resource: wgpu::BindingResource::Buffer(BufferBinding {
                    buffer: &matrix_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
            label: Some("Loading screen projection matrix bind group"),
        };

        let matrix_bind_group = device.create_bind_group(&matrix_bind_group_descriptor);

        (matrix_buffer, matrix_bind_group, matrix_bind_group_layout)
    }
}
