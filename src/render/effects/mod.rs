use crate::render::device::get_device;
use crate::render::effects::bloom::BloomPostProcessingEffect;
use crate::render::effects::gaussian::GaussianBlurPostProcessingEffect;
use crate::render::effects::merge::MergePostProcessingEffect;
use crate::render::effects::ssao::SSAOEffect;
use crate::render::{get_swapchain_size, get_texture_format};
use crate::services::settings_service::SettingsService;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use wgpu::{
    BindGroup, CommandEncoder, Device, ImageSubresourceRange, Queue, Texture, TextureAspect,
    TextureDimension, TextureUsages,
};

pub mod bloom;
pub mod gaussian;
pub mod merge;
pub mod ssao;

lazy_static! {
    static ref DROP_TEXTURES: Mutex<bool> = Mutex::new(false);
}

pub struct EffectPasses {
    pub bloom: Arc<BloomPostProcessingEffect>,
    pub ssao: Arc<SSAOEffect>,

    pub effect_gaussian: Arc<GaussianBlurPostProcessingEffect>,
    pub effect_merge: Arc<MergePostProcessingEffect>,

    buffers: Vec<SCTexture>,
    dirty_buffers: Vec<SCTexture>,
}

// A struct the same size as the swapchain
pub struct SCTexture {
    texture: Texture,
}

impl Drop for SCTexture {
    fn drop(&mut self) {
        // Check if we should log this
        if !*DROP_TEXTURES.lock().unwrap() {
            log_warn!("SCTexture dropped");
        }
    }
}

impl Deref for SCTexture {
    type Target = Texture;

    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}

impl EffectPasses {
    pub fn new(queue: &mut Queue, _settings: &SettingsService) -> EffectPasses {
        let bloom = Arc::new(BloomPostProcessingEffect::new());
        let ssao = Arc::new(SSAOEffect::new(queue));
        let effect_gaussian = Arc::new(GaussianBlurPostProcessingEffect::new());
        let effect_merge = Arc::new(MergePostProcessingEffect::new());

        EffectPasses {
            bloom,
            ssao,
            effect_gaussian,
            effect_merge,
            buffers: vec![],
            dirty_buffers: vec![],
        }
    }

    pub fn apply_bloom(
        &mut self,
        encoder: &mut CommandEncoder,
        bloom_texture: &Texture,
        frame: &Texture,
    ) {
        self.bloom
            .clone()
            .create_bloom_effect(self, encoder, bloom_texture, frame)
    }

    pub fn apply_ssao(
        &mut self,
        encoder: &mut CommandEncoder,
        normals: &Texture,
        positions: &Texture,
        projection_bind_group: &BindGroup,
    ) -> SCTexture {
        self.ssao
            .clone()
            .apply_ssao(self, encoder, normals, positions, projection_bind_group)
    }

    pub fn get_buffer(&mut self) -> SCTexture {
        if self.buffers.len() == 0 {
            let texture_descriptor = wgpu::TextureDescriptor {
                label: Some("SCTexture buffer texture"),
                size: get_swapchain_size(),
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: get_texture_format(),
                usage: TextureUsages::RENDER_ATTACHMENT
                    | TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::COPY_SRC,
            };

            SCTexture {
                texture: get_device().create_texture(&texture_descriptor),
            }
        } else {
            self.buffers.pop().unwrap()
        }
    }

    pub fn return_buffer(&mut self, texture: SCTexture) {
        self.dirty_buffers.push(texture);
    }

    pub fn clean_buffers(&mut self, encoder: &mut CommandEncoder) {
        while self.dirty_buffers.len() != 0 {
            let texture = self.dirty_buffers.pop().unwrap();

            encoder.clear_texture(
                &*texture,
                &ImageSubresourceRange {
                    aspect: TextureAspect::All,
                    base_mip_level: 0,
                    mip_level_count: None,
                    base_array_layer: 0,
                    array_layer_count: None,
                },
            );

            self.buffers.push(texture);
        }
    }

    // Force regeneration of buffers
    pub fn resize_buffers(&mut self) {
        // Set flag allowing for textures to be dropped without warning
        *DROP_TEXTURES.lock().unwrap() = true;
        self.buffers = Vec::new();
        *DROP_TEXTURES.lock().unwrap() = false;
    }
}
