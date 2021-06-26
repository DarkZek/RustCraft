use crate::services::asset_service::AssetService;
use wgpu::{
    BindGroup, BindGroupLayout, Device, Sampler, Texture, TextureAspect, TextureFormat,
    TextureSampleType, TextureViewDescriptor, TextureViewDimension,
};

impl AssetService {
    /// Create the information for the gpu to know how to deal with the atlas
    pub fn generate_atlas_bindings(
        device: &Device,
        diffuse_texture: &Texture,
        diffuse_sampler: &Sampler,
    ) -> (BindGroupLayout, BindGroup) {
        let diffuse_texture_view = diffuse_texture.create_view(&TextureViewDescriptor {
            label: Some("Asset Service Texture Atlas Texture View"),
            format: Some(TextureFormat::Rgba8UnormSrgb),
            dimension: Some(TextureViewDimension::D2),
            aspect: TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Asset Service Texture Atlas Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: false },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            filtering: false,
                            comparison: true,
                        },
                        count: None,
                    },
                ],
            });

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            label: Some("Asset Service Texture Atlas Bind Group"),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(diffuse_sampler),
                },
            ],
        });

        (texture_bind_group_layout, texture_bind_group)
    }
}
