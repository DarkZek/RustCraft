use crate::render::camera::Camera;
use nalgebra::Matrix4;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device};
use zerocopy::{AsBytes, FromBytes};

#[repr(C)]
#[derive(Debug, Copy, Clone, AsBytes, FromBytes)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
}

unsafe impl bytemuck::Zeroable for Uniforms {}
unsafe impl bytemuck::Pod for Uniforms {}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            view_proj: Matrix4::zeros().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &mut Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }

    pub fn create_uniform_buffers(self, device: &Device) -> (Buffer, BindGroupLayout, BindGroup) {
        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: &bytemuck::cast_slice(&self.view_proj),
            usage: wgpu::BufferUsage::UNIFORM
                | wgpu::BufferUsage::COPY_DST
                | wgpu::BufferUsage::COPY_SRC,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    uniform_buffer.slice(0..std::mem::size_of_val(&self) as wgpu::BufferAddress),
                ),
            }],
            label: None,
        });

        (
            uniform_buffer,
            uniform_bind_group_layout,
            uniform_bind_group,
        )
    }
}
