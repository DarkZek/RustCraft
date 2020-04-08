use crate::render::RenderState;
use winit::window::Window;
use wgpu::{AdapterInfo, Device, Queue, Surface};
use winit::dpi::PhysicalSize;

impl RenderState {
    pub fn get_devices(window: &Window) -> (PhysicalSize<u32>, Surface, AdapterInfo, Device, Queue) {
        let size = window.inner_size();

        let surface = wgpu::Surface::create(window);

        let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            backends: wgpu::BackendBit::all(),
        }).unwrap();

        let gpu_info = adapter.get_info();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: true,
            },
            limits: Default::default(),
        });

        (size, surface, gpu_info, device, queue)
    }
}