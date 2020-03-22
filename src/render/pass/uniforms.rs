use crate::render::camera::Camera;
use wgpu::{Device, BindGroupLayout, Buffer, BindGroup};
use zerocopy::{AsBytes, FromBytes};
use cgmath::SquareMatrix;

#[repr(C)]
#[derive(Debug, Copy, Clone, AsBytes, FromBytes)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4]
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }

    pub fn create_uniform_buffers(self, device: &Device) -> (Buffer, BindGroupLayout, BindGroup) {

        let uniform_buffer = device
            // The COPY_DST part will be important later
            .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&[self]);

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false
                    },
                }
            ]
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buffer,
                        range: 0..std::mem::size_of_val(&self) as wgpu::BufferAddress,
                    }
                }
            ],
        });

        (uniform_buffer, uniform_bind_group_layout, uniform_bind_group)
    }
}



