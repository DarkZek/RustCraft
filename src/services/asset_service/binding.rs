use crate::services::asset_service::AssetService;
use wgpu::{BindGroup, BindGroupLayout, Device, Sampler, Texture, TextureComponentType};

impl AssetService {
    /// Create the information for the gpu to know how to deal with the atlas
    pub fn generate_atlas_bindings(
        device: &Device,
        diffuse_texture: &Texture,
        diffuse_sampler: &Sampler,
    ) -> (BindGroupLayout, BindGroup) {
        let diffuse_texture_view = diffuse_texture.create_default_view();

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::SampledTexture {
                            multisampled: false,
                            dimension: wgpu::TextureViewDimension::D2Array,
                            component_type: TextureComponentType::Float,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler { comparison: true },
                        count: None,
                    },
                ],
            });

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            label: None,
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
