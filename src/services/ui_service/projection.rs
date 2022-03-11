use crate::render::device::get_device;
use crate::render::RenderState;
use crate::services::ui_service::UIService;
use crate::services::ServicesContext;
use nalgebra::{Matrix4, Orthographic3};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BindGroup, BindGroupLayout, Buffer, BufferBinding, BufferBindingType};
use winit::dpi::PhysicalSize;

impl UIService {
    /// Sets up the orthographic projection matrix for the UI render pipeline. This sets the size of the projection to be the same as the window dimensions.
    pub fn setup_ui_projection_matrix(
        context: &mut ServicesContext,
    ) -> (Buffer, BindGroup, BindGroupLayout) {
        log!(
            "Setting up screen with size: {}",
            format!("{}x{}", context.size.width, context.size.height)
        );

        let projection = Orthographic3::new(
            -(context.size.width as f32 / 2.0),
            context.size.width as f32 / 2.0,
            context.size.height as f32 / 2.0,
            -(context.size.height as f32 / 2.0),
            0.1,
            10.0,
        );

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
            label: Some("Main UI Projection Matrix Bind Group Layout"),
        };

        let matrix: Matrix4<f32> = projection.into();

        let matrix_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
            label: Some("Main UI Projection Matrix Buffer"),
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
            label: Some("Main UI Projection Matrix Bind Group"),
        };

        let matrix_bind_group = get_device().create_bind_group(&matrix_bind_group_descriptor);

        (matrix_buffer, matrix_bind_group, matrix_bind_group_layout)
    }

    pub fn update_ui_projection_matrix(&mut self, render: &RenderState, size: &PhysicalSize<u32>) {
        let projection = Orthographic3::new(
            -(size.width as f32 / 2.0),
            size.width as f32 / 2.0,
            size.height as f32 / 2.0,
            -(size.height as f32 / 2.0),
            0.1,
            10.0,
        );

        let mut matrix: Matrix4<f32> = projection.into();
        matrix = matrix;

        let mut encoder = get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Main Projection Matrix Command Encoder"),
        });

        let matrix_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
            label: Some("Main ui projection matrix buffer"),
            contents: &bytemuck::cast_slice(matrix.as_slice()),
            usage: wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        self.fonts.resized(&size);

        encoder.copy_buffer_to_buffer(
            &matrix_buffer,
            0x0,
            &self.projection_buffer,
            0x0,
            std::mem::size_of_val(&matrix) as wgpu::BufferAddress,
        );

        render.queue.submit(Some(encoder.finish()));
    }
}
