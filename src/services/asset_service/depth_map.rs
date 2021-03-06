use wgpu::{
    Extent3d, Sampler, Texture, TextureAspect, TextureDimension, TextureView,
    TextureViewDescriptor, TextureViewDimension,
};

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub fn create_depth_texture(
    device: &wgpu::Device,
    sc_desc: &wgpu::SwapChainDescriptor,
) -> (Texture, TextureView, Sampler) {
    let sampler_descriptor = wgpu::SamplerDescriptor {
        label: Some("Main Render Depth Map"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        lod_min_clamp: -100.0,
        lod_max_clamp: 100.0,
        compare: Some(wgpu::CompareFunction::Always),
        anisotropy_clamp: None,
        border_color: None,
    };

    let texture_descriptor = wgpu::TextureDescriptor {
        label: Some("Main depth map texture"),
        size: Extent3d {
            width: sc_desc.width,
            height: sc_desc.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: DEPTH_FORMAT,
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT | wgpu::TextureUsage::SAMPLED,
    };

    let texture = device.create_texture(&texture_descriptor);
    let view = texture.create_view(&TextureViewDescriptor {
        label: Some("Main depth map texture view"),
        format: Some(DEPTH_FORMAT),
        dimension: Some(TextureViewDimension::D2),
        aspect: TextureAspect::All,
        base_mip_level: 0,
        base_array_layer: 0,
        array_layer_count: None,
        mip_level_count: None,
    });
    let sampler = device.create_sampler(&sampler_descriptor);

    (texture, view, sampler)
}
