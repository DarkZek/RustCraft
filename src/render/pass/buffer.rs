use bytemuck::Pod;
use std::sync::Arc;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindingResource, Buffer,
    BufferBinding, BufferUsage, Device,
};

pub struct BufferGroup {
    pub buffer: Option<Buffer>,
    pub bind_group: Option<BindGroup>,
    pub bind_group_layout: Arc<BindGroupLayout>,
    usage: BufferUsage,
}

impl BufferGroup {
    pub fn new(
        device: &Device,
        descriptor: &BindGroupLayoutDescriptor,
        usage: BufferUsage,
    ) -> BufferGroup {
        let bind_group_layout = device.create_bind_group_layout(descriptor);

        BufferGroup {
            buffer: None,
            bind_group: None,
            bind_group_layout: Arc::new(bind_group_layout),
            usage,
        }
    }

    pub fn with_layout(bind_group_layout: Arc<BindGroupLayout>, usage: BufferUsage) -> BufferGroup {
        BufferGroup {
            buffer: None,
            bind_group: None,
            bind_group_layout,
            usage,
        }
    }

    pub fn generate<A: Pod>(&mut self, data: &[A], device: &Device) {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Unknown buffer group"),
            contents: &bytemuck::cast_slice(data),
            usage: self.usage,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Unknown bind group"),
            layout: &self.bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        self.bind_group = Some(bind_group);
        self.buffer = Some(buffer);
    }
}
