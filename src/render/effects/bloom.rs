use crate::render::effects::EffectPasses;


use wgpu::{
    CommandEncoder, Device, Texture, TextureViewDescriptor,
};

pub struct BloomPostProcessingEffect {}

impl BloomPostProcessingEffect {
    pub fn new(_device: &Device) -> BloomPostProcessingEffect {
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
