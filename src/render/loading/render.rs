use crate::render::device::get_device;
use crate::render::get_texture_format;
use crate::render::loading::LoadingScreen;
use nalgebra::{Matrix4, Orthographic3};
use rc_ui::vertex::UIVertex;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupLayout, BlendComponent, BlendState, Buffer, BufferBinding,
    BufferBindingType, RenderPipeline, VertexState,
};
use winit::dpi::PhysicalSize;

impl LoadingScreen {
    pub fn generate_loading_render_pipeline(
        bind_group_layouts: &[&BindGroupLayout],
    ) -> RenderPipeline {
        let render_pipeline_layout =
            get_device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Loading screen pipeline layout descriptor"),
                bind_group_layouts,
                push_constant_ranges: &[],
            });

        let vs_module = get_device()
            .create_shader_module(&wgpu::include_spirv!("../../shaders/loading_vert.spv"));
        let fs_module = get_device()
            .create_shader_module(&wgpu::include_spirv!("../../shaders/loading_frag.spv"));

        get_device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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

    pub fn setup_ui_projection_matrix(
        size: PhysicalSize<u32>,
    ) -> (Buffer, BindGroup, BindGroupLayout) {
        let ratio = size.width as f32 / size.height as f32;

        let projection = Orthographic3::new(-ratio, ratio, -1.0, 1.0, 0.1, 10.0);

        let matrix_binding_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
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

        let matrix_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
            label: Some("Loading screen projection matrix buffer"),
            contents: &bytemuck::cast_slice(matrix.as_slice()),
            usage: wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        let matrix_bind_group_layout =
            get_device().create_bind_group_layout(&matrix_binding_layout_descriptor);

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

        let matrix_bind_group = get_device().create_bind_group(&matrix_bind_group_descriptor);

        (matrix_buffer, matrix_bind_group, matrix_bind_group_layout)
    }
}
