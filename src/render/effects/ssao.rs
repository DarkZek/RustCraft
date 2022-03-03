use crate::render::effects::EffectPasses;
use wgpu::{CommandEncoder, TextureView};

pub struct SSAOEffect {}

impl SSAOEffect {
    pub fn apply_ssao(
        &self,
        _effect_passes: &mut EffectPasses,
        _encoder: &mut CommandEncoder,
        _src: &TextureView,
    ) {
    }
}
