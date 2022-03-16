use crate::render::device::get_device;
use crate::render::{get_swapchain_size, get_texture_format};
use std::ops::Deref;
use std::sync::Mutex;
use wgpu::{
    CommandEncoder, ImageSubresourceRange, Texture, TextureAspect, TextureDimension, TextureUsages,
};

lazy_static! {
    static ref DROP_TEXTURES: Mutex<bool> = Mutex::new(false);
}

pub struct TextureBufferPool {
    buffers: Vec<SCTexture>,
    dirty_buffers: Vec<SCTexture>,
}

impl TextureBufferPool {
    pub fn new() -> TextureBufferPool {
        TextureBufferPool {
            buffers: vec![],
            dirty_buffers: vec![],
        }
    }

    pub fn get_buffer(&mut self) -> SCTexture {
        if self.buffers.len() == 0 {
            let texture_descriptor = wgpu::TextureDescriptor {
                label: Some("SCTexture buffer texture"),
                size: get_swapchain_size(),
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: *get_texture_format(),
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
        self.dirty_buffers = Vec::new();
        *DROP_TEXTURES.lock().unwrap() = false;
    }
}

impl Default for TextureBufferPool {
    fn default() -> Self {
        todo!()
    }
}

/// A texture that logs when dropped
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
