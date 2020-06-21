use crate::services::asset_service::atlas::TextureAtlasIndex;
use crate::services::asset_service::AssetService;
use crate::services::chunk_service::mesh::UIVertex;
use crate::services::ui_service::fonts::text::Text;
use crate::services::ui_service::fonts::text_builder::TextBuilder;
use crate::services::ui_service::fonts::variable_width::generate_variable_width_map;
use wgpu::{Buffer, Device};
use winit::dpi::PhysicalSize;

pub mod text;
pub mod text_builder;
pub mod variable_width;

/// Fonts Manager is a subsystem of the User Interface Service.
/// It's job is to manage fonts and allow other services to easily create new fonts on the screen as well as update and delete them.
pub struct FontsManager {
    texts: Vec<Text>,
    pub total_vertices: Vec<UIVertex>,
    pub total_indices: Vec<u16>,
    changed: bool,
    pub total_vertex_buffer: Option<Buffer>,
    pub total_indices_buffer: Option<Buffer>,
    atlas_coords: FontAtlasIndexs,
    character_widths: [u8; 127],
    size: PhysicalSize<u32>,
}

pub struct FontAtlasIndexs {
    ascii: TextureAtlasIndex,
}

/// The struct given to the service from the font manager. It's easier with lifetimes if we don't directly give them access to the object.
pub struct TextView {
    id: usize,
}

// How many letters per row inside texture
pub const FONT_TEXTURE_SIZE: f32 = 16.0;
pub const LETTER_SPACING: f32 = 0.2;

impl FontsManager {

    /// Sets up the font manager, including getting font asset locations inside the texture atlas and calculating the variable width font distancing.
    pub fn new(assets: &AssetService, size: PhysicalSize<u32>) -> FontsManager {
        let ascii_atlas_coords = assets
            .atlas_index
            .as_ref()
            .unwrap()
            .get("textures/font/ascii")
            .unwrap()
            .clone();

        let character_widths = generate_variable_width_map(assets);

        FontsManager {
            texts: Vec::new(),
            total_vertices: Vec::new(),
            total_indices: Vec::new(),
            changed: true,
            total_vertex_buffer: None,
            total_indices_buffer: None,
            atlas_coords: FontAtlasIndexs {
                ascii: ascii_atlas_coords,
            },
            character_widths,
            size,
        }
    }

    pub fn create_text(&mut self) -> TextBuilder {
        TextBuilder::new(self)
    }

    fn add_text(&mut self, mut text: Text) -> TextView {
        text.generate_text_mesh((&self.atlas_coords, &self.character_widths), &self.size);
        self.texts.push(text);

        TextView {
            id: self.texts.len(),
        }
    }

    pub fn edit_text(&mut self, text: &TextView, new_text: String) {
        self.changed = true;
        let text = self.texts.get_mut(text.id - 1).unwrap();
        text.text = new_text;
        text.generate_text_mesh((&self.atlas_coords, &self.character_widths), &self.size);
    }

    /// Totals the vertices if any of the text has changed
    pub fn total(&mut self, device: &Device) {
        if self.changed {
            self.changed = false;

            self.total_vertices.clear();
            self.total_indices.clear();

            for text in &mut self.texts {
                let starting_vertices = self.total_vertices.len();
                for index in &text.indices {
                    self.total_indices.push(index + starting_vertices as u16);
                }

                self.total_vertices.append(&mut text.vertices.clone());
            }

            self.total_vertex_buffer = Some(
                device
                    .create_buffer_mapped(self.total_vertices.len(), wgpu::BufferUsage::VERTEX)
                    .fill_from_slice(self.total_vertices.as_mut_slice()),
            );

            self.total_indices_buffer = Some(
                device
                    .create_buffer_mapped(self.total_indices.len(), wgpu::BufferUsage::INDEX)
                    .fill_from_slice(self.total_indices.as_mut_slice()),
            );
        }
    }

    /// Recreate on resize
    pub fn resized(&mut self, size: &PhysicalSize<u32>, device: &Device) {
        self.size = size.clone();

        for text in self.texts.iter_mut() {
            text.generate_text_mesh((&self.atlas_coords, &self.character_widths), &size);
        }

        self.changed = true;
        self.total(device);
    }
}
