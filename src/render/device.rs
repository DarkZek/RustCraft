use crate::render::RenderState;
use futures::executor::block_on;
use wgpu::{AdapterInfo, BackendBit, Device, Instance, Queue, Surface};
use winit::dpi::PhysicalSize;
use winit::window::Window;

impl RenderState {
    /// Gets a gpu devices we will use
    pub fn get_devices(
        window: &Window,
    ) -> (PhysicalSize<u32>, Surface, AdapterInfo, Device, Queue) {
        let size = window.inner_size();

        let wgpu = Instance::new(BackendBit::BROWSER_WEBGPU | BackendBit::VULKAN);

        let surface = unsafe { wgpu.create_surface(window) };

        let adapter = block_on(wgpu.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
        }))
        .unwrap();

        let gpu_info = adapter.get_info();

        let (device, queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: Default::default(),
                limits: Default::default(),
                shader_validation: false,
            },
            None,
        ))
        .unwrap();

        (size, surface, gpu_info, device, queue)
    }
}
