use crate::services::asset_service::atlas::ATLAS_LOOKUPS;
use crate::services::asset_service::index::TextureAtlasIndex;
use crate::services::ui_service::draw::draw_sprite;
use crate::services::ui_service::meshdata::UIMeshData;
use std::collections::HashMap;
use wgpu::Device;
use winit::dpi::PhysicalSize;

/// Image Manager is a subsystem of the User Interface Service.
/// It's job is to manage images and allow other services to easily create new images on the screen as well as update and delete them.
pub struct ImageManager {
    images: HashMap<usize, Image>,
    pub model: UIMeshData,
    pub size: PhysicalSize<u32>,
    build: bool,
}

impl ImageManager {
    pub fn new(size: PhysicalSize<u32>) -> ImageManager {
        ImageManager {
            images: HashMap::new(),
            model: UIMeshData::new(),
            size,
            build: false,
        }
    }

    pub fn create_image(&mut self, image: &str) -> ImageBuilder {
        let atlas_lookup = ATLAS_LOOKUPS
            .get()
            .unwrap()
            .get(image)
            .unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap())
            .clone();
        ImageBuilder {
            image: Some(Image {
                name: image.to_string(),
                atlas_lookup,
                fullscreen: false,
                flipped: false,
                ty: ImageType::STATIC([0.0; 2], [10.0; 2]),
            }),
            manager: self,
        }
    }

    pub fn add_image(&mut self, image: Image) -> ImageView {
        let id = rand::random::<usize>();
        self.images.insert(id, image);

        self.build = true;

        ImageView { id }
    }

    pub fn delete_image(&mut self, image: ImageView) {
        self.images.remove(&image.id);
        self.build = true;
    }

    pub fn build(&mut self, device: &Device) {
        if !self.build {
            return;
        }

        self.model.clear();

        for (id, image) in self.images.iter() {
            let atlas_lookup = if image.flipped {
                image.atlas_lookup.flipped()
            } else {
                image.atlas_lookup
            };

            match image.ty {
                ImageType::STATIC(pos, scale) => {
                    draw_sprite(
                        &mut self.model.total_vertices,
                        &mut self.model.total_indices,
                        pos,
                        scale,
                        atlas_lookup,
                        None,
                    );
                }
                ImageType::BACKGROUND(scale) => {
                    let h = (self.size.height as i64) / scale as i64;
                    let w = (self.size.width as i64) / scale as i64;
                    for x in -w..(w + 1) {
                        for y in -h..(h + 1) {
                            draw_sprite(
                                &mut self.model.total_vertices,
                                &mut self.model.total_indices,
                                [
                                    ((x * scale as i64) - (self.size.width as i64 / 2)) as f32,
                                    ((y * scale as i64) - (self.size.height as i64 / 2)) as f32,
                                ],
                                [scale as f32; 2],
                                atlas_lookup,
                                None,
                            );
                        }
                    }
                }
            }
        }

        self.model.build_buf(device);
    }
}

pub struct ImageBuilder<'a> {
    image: Option<Image>,
    manager: &'a mut ImageManager,
}

impl ImageBuilder<'_> {
    pub fn build(mut self) -> ImageView {
        self.manager.add_image(self.image.take().unwrap())
    }
    pub fn set_fullscreen(mut self, fullscreen: bool) -> Self {
        self.image.as_mut().unwrap().fullscreen = fullscreen;
        self
    }
    pub fn set_type(mut self, ty: ImageType) -> Self {
        self.image.as_mut().unwrap().ty = ty;
        self
    }
    pub fn set_flipped(mut self, flipped: bool) -> Self {
        self.image.as_mut().unwrap().flipped = flipped;
        self
    }
}

pub struct Image {
    name: String,
    atlas_lookup: TextureAtlasIndex,
    fullscreen: bool,
    flipped: bool,
    ty: ImageType,
}

pub enum ImageType {
    STATIC([f32; 2], [f32; 2]),
    BACKGROUND(u32),
}

pub struct ImageView {
    id: usize,
}

impl ImageView {
    pub fn clone(&self) -> ImageView {
        ImageView { id: self.id }
    }
}
