use crate::render::effects::EffectPasses;
use wgpu::{CommandEncoder, TextureView};

pub struct SSAOEffect {}

impl SSAOEffect {
    pub fn apply_ssao(
        &self,
        effect_passes: &mut EffectPasses,
        encoder: &mut CommandEncoder,
        src: &TextureView,
    ) {
    }
}
