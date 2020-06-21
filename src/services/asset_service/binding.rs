use crate::services::asset_service::AssetService;
use wgpu::{BindGroup, BindGroupLayout, Device, Sampler, Texture};

impl AssetService {
    pub fn generate_atlas_bindings(
        device: &Device,
        diffuse_texture: &Texture,
        diffuse_sampler: &Sampler,
    ) -> (BindGroupLayout, BindGroup) {
        let diffuse_texture_view = diffuse_texture.create_default_view();

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutBinding {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::SampledTexture {
                            multisampled: false,
                            dimension: wgpu::TextureViewDimension::D2Array,
                        },
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler,
                    },
                ],
            });

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(diffuse_sampler),
                },
            ],
        });

        (texture_bind_group_layout, texture_bind_group)
    }
}
