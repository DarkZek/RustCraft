use crate::render::get_device;
use crate::UIRenderPipeline;
use nalgebra::{Matrix4, Orthographic3};
use rc_logging::log;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BindGroup, BindGroupLayout, Buffer, BufferBinding, BufferBindingType, Extent3d, Queue};

impl UIRenderPipeline {
    /// Sets up the orthographic projection matrix for the UI render pipeline. This sets the size of the projection to be the same as the window dimensions.
    pub fn setup_ui_projection_matrix(size: Extent3d) -> (Buffer, BindGroup, BindGroupLayout) {
        log!(
            "Setting up UI screen with size: {}",
            format!("{}x{}", size.width, size.height)
        );

        let projection = Orthographic3::new(
            -(size.width as f32 / 2.0),
            size.width as f32 / 2.0,
            size.height as f32 / 2.0,
            -(size.height as f32 / 2.0),
            0.1,
            10.0,
        );

        let matrix: Matrix4<f32> = projection.into();

        let projection_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
            label: Some("UI Projection Matrix Buffer"),
            contents: &bytemuck::cast_slice(matrix.as_slice()),
            usage: wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        let projection_bind_group_layout =
            get_device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                label: Some("UI Projection Matrix Bind Group Layout"),
            });

        let projection_bind_group = get_device().create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &projection_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(BufferBinding {
                    buffer: &projection_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
            label: Some("UI Projection Matrix Bind Group"),
        });

        (
            projection_buffer,
            projection_bind_group,
            projection_bind_group_layout,
        )
    }

    pub fn update_ui_projection_matrix(&self, queue: &mut Queue, size: &Extent3d) {
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
            label: Some("UI Projection Matrix Command Encoder"),
        });

        let matrix_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
            label: Some("UI projection matrix buffer"),
            contents: &bytemuck::cast_slice(matrix.as_slice()),
            usage: wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        encoder.copy_buffer_to_buffer(
            &matrix_buffer,
            0x0,
            &self.projection_buffer,
            0x0,
            std::mem::size_of_val(&matrix) as wgpu::BufferAddress,
        );

        queue.submit(Some(encoder.finish()));
    }
}
