use crate::render::post_processing::bloom::BloomPostProcessingEffect;
use crate::render::post_processing::merge::MergePostProcessingEffect;
use crate::services::settings_service::SettingsService;
use wgpu::{Device, SurfaceConfiguration};

pub mod bloom;
pub mod merge;

pub struct PostProcessingEffects {
    pub bloom: BloomPostProcessingEffect,
    pub merge: MergePostProcessingEffect,
}

impl PostProcessingEffects {
    pub fn new(
        settings: &SettingsService,
        device: &Device,
        surface: &SurfaceConfiguration,
    ) -> PostProcessingEffects {
        let bloom = BloomPostProcessingEffect::new(device, surface);
        let merge = MergePostProcessingEffect::new(device, surface);

        PostProcessingEffects { bloom, merge }
    }

    pub fn
}
