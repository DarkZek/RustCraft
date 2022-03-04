use fnv::{FnvBuildHasher, FnvHashMap};
use std::collections::HashMap;

use wgpu::Device;
use winit::dpi::PhysicalSize;

use crate::services::asset_service::index::TextureAtlasIndex;
use crate::services::asset_service::AssetService;
use crate::services::ui_service::fonts::text::Text;
use crate::services::ui_service::fonts::text_builder::TextBuilder;
use crate::services::ui_service::fonts::variable_width::generate_variable_width_map;
use crate::services::ui_service::meshdata::UIMeshData;

pub mod system;
pub mod text;
pub mod text_builder;
pub mod variable_width;

/// Fonts Manager is a subsystem of the User Interface Service.
/// It's job is to manage fonts and allow other services to easily create new fonts on the screen as well as update and delete them.
pub struct FontsManager {
    pub texts: HashMap<usize, Text, FnvBuildHasher>,
    pub changed: bool,
    pub model: UIMeshData,
    pub atlas_coords: FontAtlasIndexs,
    pub character_widths: [u8; 127],
    pub size: PhysicalSize<u32>,
}

pub struct FontAtlasIndexs {
    ascii: TextureAtlasIndex,
}

/// The struct given to the service from the font manager. It's easier with lifetimes if we don't directly give them access to the object.
#[derive(Copy, Clone)]
pub struct TextView {
    id: usize,
}

// How many letters per row inside texture
pub const FONT_TEXTURE_SIZE: f32 = 16.0;
pub const LETTER_SPACING: f32 = 0.25;

impl FontsManager {
    /// Sets up the font manager, including getting font asset locations inside the texture atlas and calculating the variable width font distancing.
    pub fn new(assets: &AssetService, size: PhysicalSize<u32>) -> FontsManager {
        let ascii_atlas_coords = assets
            .atlas_index
            .as_ref()
            .unwrap()
            .get("font/ascii")
            .unwrap()
            .clone();

        let character_widths = generate_variable_width_map(assets);

        FontsManager {
            texts: FnvHashMap::default(),
            changed: true,
            atlas_coords: FontAtlasIndexs {
                ascii: ascii_atlas_coords,
            },
            character_widths,
            size,
            model: UIMeshData::new(),
        }
    }

    pub fn create_text(&mut self) -> TextBuilder {
        TextBuilder::new(self)
    }

    fn add_text(&mut self, mut text: Text) -> TextView {
        let id = rand::random::<usize>();

        self.changed = true;
        text.generate_text_mesh((&self.atlas_coords, &self.character_widths), &self.size);
        self.texts.insert(id, text);

        TextView { id }
    }

    pub fn edit_text(&mut self, text: &TextView, new_text: String) {
        // Ensure text actually changed
        if new_text == self.texts.get_mut(&text.id).unwrap().text {
            return;
        }

        self.changed = true;
        let text = self.texts.get_mut(&text.id).unwrap();
        text.text = new_text;
        text.generate_text_mesh((&self.atlas_coords, &self.character_widths), &self.size);
    }

    /// Totals the vertices if any of the text has changed
    pub fn total(&mut self, device: &Device) {
        if self.changed {
            self.changed = false;

            self.model.clear();

            for (_, text) in &mut self.texts {
                let starting_vertices = self.model.total_vertices.len();
                for index in &text.indices {
                    self.model
                        .total_indices
                        .push(index + starting_vertices as u16);
                }

                self.model.total_vertices.append(&mut text.vertices.clone());
            }

            self.model.build_buf(device);
        }
    }

    /// Recreate on resize
    pub fn resized(&mut self, size: &PhysicalSize<u32>, device: &Device) {
        self.size = size.clone();

        for (_, text) in self.texts.iter_mut() {
            text.generate_text_mesh((&self.atlas_coords, &self.character_widths), &size);
        }

        self.changed = true;
        self.total(device);
    }
}
