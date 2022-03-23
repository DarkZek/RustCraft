use crate::render::get_device;
use crate::UIRenderPipeline;
use nalgebra::{
    Isometry3, Matrix3, Matrix4, Orthographic3, Point3, Translation2, Translation3, Vector3,
};
use rc_logging::log;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupLayout, Buffer, BufferBinding, BufferBindingType, CommandEncoder, Extent3d,
    Queue,
};

impl UIRenderPipeline {
    /// Sets up the orthographic projection matrix for the UI render pipeline. This sets the size of the projection to be the same as the window dimensions.
    pub fn setup_ui_projection_matrix(size: Extent3d) -> (Buffer, BindGroup, BindGroupLayout) {
        let projection_buffer =
            Self::setup_ui_projection_matrix_buffer(size.width as f32, size.height as f32);

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

    /// Sets up a vec2 with the size of the viewport
    pub fn setup_ui_projection_matrix_buffer(x: f32, y: f32) -> Buffer {
        get_device().create_buffer_init(&BufferInitDescriptor {
            label: Some("UI Projection Matrix Buffer"),
            contents: &bytemuck::cast_slice(&[x, y]),
            usage: wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        })
    }

    pub fn update_ui_projection_matrix(&self, encoder: &mut CommandEncoder, size: &Extent3d) {
        let buffer = Self::setup_ui_projection_matrix_buffer(size.width as f32, size.height as f32);

        encoder.copy_buffer_to_buffer(
            &buffer,
            0x0,
            &self.projection_buffer,
            0x0,
            std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
        );
    }
}
