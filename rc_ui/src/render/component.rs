use crate::render::pipeline::UIRenderPipeline;
use crate::render::{get_device, get_swapchain_format};
use wgpu::Extent3d;

impl UIRenderPipeline {
    pub fn create_component_texture(&mut self, width: u32, height: u32) {
        let diffuse_texture = get_device().create_texture(&wgpu::TextureDescriptor {
            label: Some("UI Component texture"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 0,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: *get_swapchain_format(),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });
    }
}
