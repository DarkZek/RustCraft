use crate::render::effects::EffectPasses;
use crate::render::vertices::UIVertex;
use crate::render::{get_texture_format, VERTICES_COVER_SCREEN};
use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
    CommandEncoder, Device, RenderPassColorAttachment, RenderPipeline, SamplerBindingType,
    SamplerDescriptor, ShaderStages, Texture, TextureSampleType, TextureViewDescriptor,
    TextureViewDimension, VertexState,
};

pub struct BloomPostProcessingEffect {}

impl BloomPostProcessingEffect {
    pub fn new(device: &Device) -> BloomPostProcessingEffect {
        BloomPostProcessingEffect {}
    }

    pub fn create_bloom_effect(
        &self,
        effect_passes: &mut EffectPasses,
        encoder: &mut CommandEncoder,
        bloom_texture: &Texture,
        frame: &Texture,
    ) {
        // Blur texture
        effect_passes.effect_gaussian.clone().apply_gaussian_blur(
            effect_passes,
            encoder,
            &bloom_texture,
            frame,
            20,
        );

        // Merge it into the frame
        effect_passes.effect_merge.clone().merge(
            effect_passes,
            encoder,
            &bloom_texture.create_view(&TextureViewDescriptor::default()),
            frame,
        );
    }
}
