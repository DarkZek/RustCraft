use crate::render::RenderState;
use wgpu::{AdapterInfo, Device, Queue, Surface, BackendBit, Adapter};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use futures::executor::block_on;

impl RenderState {

    /// Gets a gpu devices we will use
    pub fn get_devices(
        window: &Window,
    ) -> (PhysicalSize<u32>, Surface, AdapterInfo, Device, Queue) {
        let size = window.inner_size();

        let surface = Surface::create(window);

        let adapter = block_on(Adapter::request(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None
        }, BackendBit::BROWSER_WEBGPU | BackendBit::VULKAN)).unwrap();

        let gpu_info = adapter.get_info();

        let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: true,
            },
            limits: Default::default(),
        }));

        (size, surface, gpu_info, device, queue)
    }
}
