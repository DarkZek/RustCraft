use crate::render::device::get_device;
use crate::render::effects::bloom::BloomPostProcessingEffect;
use crate::render::effects::buffer_pool::TextureBufferPool;
use crate::render::effects::gaussian::GaussianBlurPostProcessingEffect;
use crate::render::effects::merge::MergePostProcessingEffect;
use crate::render::effects::multiply::MultiplyPostProcessingEffect;
use crate::render::effects::ssao::SSAOEffect;
use crate::render::get_swapchain_size;
use crate::services::settings_service::SettingsService;

use wgpu::{
    BindGroup, CommandEncoder, Queue, Texture, TextureDimension, TextureFormat, TextureUsages,
};

pub mod bloom;
pub mod buffer_pool;
pub mod gaussian;
pub mod merge;
pub mod multiply;
pub mod ssao;

pub struct EffectPasses {
    pub bloom: Option<BloomPostProcessingEffect>,
    pub ssao: Option<SSAOEffect>,

    pub effect_gaussian: GaussianBlurPostProcessingEffect,
    pub effect_merge: MergePostProcessingEffect,
    pub effect_multiply: MultiplyPostProcessingEffect,

    pub normal_map: Texture,
    pub position_map: Texture,
}

impl Default for EffectPasses {
    fn default() -> Self {
        todo!()
    }
}

impl EffectPasses {
    pub fn new(queue: &mut Queue, settings: &SettingsService) -> EffectPasses {
        let bloom = if settings.config.bloom {
            Some(BloomPostProcessingEffect::new())
        } else {
            None
        };
        let ssao = if settings.config.ssao {
            Some(SSAOEffect::new(queue))
        } else {
            None
        };

        let effect_gaussian = GaussianBlurPostProcessingEffect::new();
        let effect_merge = MergePostProcessingEffect::new();
        let effect_multiply = MultiplyPostProcessingEffect::new();

        let normal_map = get_device().create_texture(&wgpu::TextureDescriptor {
            label: Some("Normal Map texture"),
            size: get_swapchain_size(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba16Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
        });

        let position_map = get_device().create_texture(&wgpu::TextureDescriptor {
            label: Some("Position Map texture"),
            size: get_swapchain_size(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba16Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
        });

        EffectPasses {
            bloom,
            ssao,
            effect_gaussian,
            effect_merge,
            effect_multiply,
            normal_map,
            position_map,
        }
    }

    pub fn apply_bloom(
        &self,
        encoder: &mut CommandEncoder,
        buffer_pool: &mut TextureBufferPool,
        bloom_texture: &Texture,
        sc: &Texture,
    ) {
        if let Some(bloom) = &self.bloom {
            bloom.create_bloom_effect(self, encoder, buffer_pool, bloom_texture, sc)
        }
    }

    pub fn apply_ssao(
        &self,
        encoder: &mut CommandEncoder,
        buffer_pool: &mut TextureBufferPool,
        projection_bind_group: &BindGroup,
        sc: &Texture,
    ) {
        if let Some(ssao) = &self.ssao {
            ssao.apply_ssao(self, encoder, buffer_pool, projection_bind_group, sc)
        }
    }

    pub fn resize(&mut self) {
        if let Some(ssao) = &mut self.ssao {
            ssao.resize();
        }

        self.normal_map = get_device().create_texture(&wgpu::TextureDescriptor {
            label: Some("Normal Map texture"),
            size: get_swapchain_size(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba16Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
        });

        self.position_map = get_device().create_texture(&wgpu::TextureDescriptor {
            label: Some("Position Map texture"),
            size: get_swapchain_size(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba16Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
        });
    }
}
