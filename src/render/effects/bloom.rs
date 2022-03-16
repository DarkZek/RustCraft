use crate::render::effects::EffectPasses;

use crate::render::effects::buffer_pool::TextureBufferPool;
use wgpu::{CommandEncoder, Texture, TextureViewDescriptor};

pub struct BloomPostProcessingEffect {}

impl BloomPostProcessingEffect {
    pub fn new() -> BloomPostProcessingEffect {
        BloomPostProcessingEffect {}
    }

    pub fn create_bloom_effect(
        &self,
        effect_passes: &EffectPasses,
        encoder: &mut CommandEncoder,
        buffer_pool: &mut TextureBufferPool,
        bloom_texture: &Texture,
        frame: &Texture,
    ) {
        // Blur texture
        effect_passes.effect_gaussian.apply_gaussian_blur(
            effect_passes,
            encoder,
            buffer_pool,
            &bloom_texture,
            frame,
            20,
        );

        // Merge it into the frame
        effect_passes.effect_merge.merge(
            effect_passes,
            encoder,
            buffer_pool,
            &bloom_texture.create_view(&TextureViewDescriptor::default()),
            frame,
        );
    }
}
