use crate::render::camera::Camera;
use crate::render::device::get_device;

use std::num::NonZeroU64;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupLayout, Buffer, BufferAddress, BufferBinding, BufferBindingType,
    BufferSize, CommandEncoder,
};
use zerocopy::{AsBytes, FromBytes};

#[repr(C)]
#[derive(Debug, Copy, Clone, AsBytes, FromBytes)]
pub struct RenderViewProjectionUniforms;

unsafe impl bytemuck::Zeroable for RenderViewProjectionUniforms {}
unsafe impl bytemuck::Pod for RenderViewProjectionUniforms {}

impl RenderViewProjectionUniforms {
    pub fn update_uniform_buffers(
        camera: &mut Camera,
        encoder: &mut CommandEncoder,
        buffer: &Buffer,
    ) {
        let (view, proj) = camera.build_view_projection_matrix();

        let view: [[f32; 4]; 4] = view.into();
        let proj: [[f32; 4]; 4] = proj.into();

        let uniform_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
            label: Some("View Projection Buffer"),
            contents: &bytemuck::cast_slice(&[view, proj]),
            usage: wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        encoder.copy_buffer_to_buffer(
            &uniform_buffer,
            0x0,
            &buffer,
            0x0,
            std::mem::size_of::<[[[f32; 4]; 4]; 2]>() as wgpu::BufferAddress,
        );
    }

    pub fn create_uniform_buffers(
        &self,
    ) -> (
        Buffer,
        BindGroupLayout,
        BindGroup,
        BindGroupLayout,
        BindGroup,
    ) {
        let uniform_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
            label: Some("View Projection uniform buffer"),
            contents: &bytemuck::cast_slice(&[[[0.0; 4]; 4]; 2]),
            usage: wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        let uniform_bind_group_layout =
            get_device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            min_binding_size: None,
                            has_dynamic_offset: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            min_binding_size: None,
                            has_dynamic_offset: false,
                        },
                        count: None,
                    },
                ],
                label: Some("View Projection uniform buffer bind group layout"),
            });

        let uniform_bind_group = get_device().create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(BufferBinding {
                        buffer: &uniform_buffer,
                        offset: 0,
                        size: Some(
                            NonZeroU64::new(std::mem::size_of::<[[[f32; 4]; 4]; 2]>() as u64)
                                .unwrap() as BufferSize,
                        ),
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(BufferBinding {
                        buffer: &uniform_buffer,
                        offset: std::mem::size_of::<[[[f32; 4]; 4]; 2]>() as BufferAddress,
                        size: None,
                    }),
                },
            ],
            label: Some("View Projection uniform buffer bind group"),
        });

        let fragment_uniform_bind_group_layout =
            get_device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            min_binding_size: None,
                            has_dynamic_offset: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            min_binding_size: None,
                            has_dynamic_offset: false,
                        },
                        count: None,
                    },
                ],
                label: Some("View Projection uniform buffer bind group layout"),
            });

        let fragment_uniform_bind_group =
            get_device().create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &fragment_uniform_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(BufferBinding {
                            buffer: &uniform_buffer,
                            offset: 0,
                            size: Some(
                                NonZeroU64::new(std::mem::size_of::<[[[f32; 4]; 4]; 2]>() as u64)
                                    .unwrap() as BufferSize,
                            ),
                        }),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Buffer(BufferBinding {
                            buffer: &uniform_buffer,
                            offset: std::mem::size_of::<[[[f32; 4]; 4]; 2]>() as BufferAddress,
                            size: None,
                        }),
                    },
                ],
                label: Some("View Projection uniform buffer bind group"),
            });

        (
            uniform_buffer,
            uniform_bind_group_layout,
            uniform_bind_group,
            fragment_uniform_bind_group_layout,
            fragment_uniform_bind_group,
        )
    }
}
