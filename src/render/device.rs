use crate::render::RenderState;
use futures::executor::block_on;
use wgpu::{Adapter, AdapterInfo, BackendBit, Device, Instance, Queue, Surface, Features};
use winit::dpi::PhysicalSize;
use winit::window::Window;

impl RenderState {
    /// Gets a gpu devices we will use
    pub fn get_devices(
        window: &Window,
    ) -> (
        PhysicalSize<u32>,
        Surface,
        AdapterInfo,
        Device,
        Queue,
        Adapter,
    ) {
        let size = window.inner_size();

        let wgpu = Instance::new(BackendBit::BROWSER_WEBGPU | BackendBit::VULKAN);

        let surface = unsafe { wgpu.create_surface(window) };

        let adapter = block_on(wgpu.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: Some(&surface),
        }))
        .unwrap();

        let gpu_info = adapter.get_info();

        let (device, queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device"),
                features: Features::default() | Features::PUSH_CONSTANTS,
                limits: wgpu::Limits {
                    max_texture_dimension_1d: 8192,
                    max_texture_dimension_2d: 8192,
                    max_texture_dimension_3d: 2048,
                    max_texture_array_layers: 2048,
                    max_bind_groups: 4,
                    max_dynamic_uniform_buffers_per_pipeline_layout: 8,
                    max_dynamic_storage_buffers_per_pipeline_layout: 4,
                    max_sampled_textures_per_shader_stage: 16,
                    max_samplers_per_shader_stage: 16,
                    max_storage_buffers_per_shader_stage: 4,
                    max_storage_textures_per_shader_stage: 4,
                    max_uniform_buffers_per_shader_stage: 12,
                    max_uniform_buffer_binding_size: 16384,
                    max_storage_buffer_binding_size: 128 << 20,
                    max_vertex_buffers: 8,
                    max_vertex_attributes: 16,
                    max_vertex_buffer_array_stride: 2048,
                    max_push_constant_size: 8,
                },
            },
            None,
        ))
        .unwrap();

        (size, surface, gpu_info, device, queue, adapter)
    }
}
