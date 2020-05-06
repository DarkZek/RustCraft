use crate::services::chunk_service::mesh::{UIVertex};
use cgmath::{Point2};
use wgpu::{Buffer, Device};
use crate::services::asset_service::atlas::TextureAtlasIndex;
use crate::services::asset_service::AssetService;

pub struct FontsManager {
    texts: Vec<Text>,
    pub(crate) total_vertices: Vec<UIVertex>,
    pub(crate) total_indices: Vec<u16>,
    changed: bool,
    pub(crate) total_vertex_buffer: Option<Buffer>,
    pub(crate) total_indices_buffer: Option<Buffer>,
    atlas_coords: FontAtlasIndexs
}

pub struct FontAtlasIndexs {
    ascii: TextureAtlasIndex
}

pub struct TextView {
    id: usize
}

// How many letters per row inside texture
const FONT_TEXTURE_SIZE: f32 = 16.0;
const LETTER_SPACING: f32 = 0.00;

impl FontsManager {
    pub fn new(assets: &AssetService) -> FontsManager {

        let ascii_atlas_coords = assets.atlas_index
            .as_ref()
            .unwrap()
            .get("textures/font/ascii")
            .unwrap()
            .clone();

        println!("{:?}", ascii_atlas_coords);

        FontsManager {
            texts: Vec::new(),
            total_vertices: Vec::new(),
            total_indices: Vec::new(),
            changed: true,
            total_vertex_buffer: None,
            total_indices_buffer: None,
            atlas_coords: FontAtlasIndexs {
                ascii: ascii_atlas_coords
            }
        }
    }

    pub fn add_text(&mut self, content: String, pos: Point2<f32>, size: f32) -> TextView {
        let mut text = Text::new(content, pos, size);
        text.generate_text_mesh(&self.atlas_coords);
        self.texts.push(text);

        TextView {
            id: self.texts.len()
        }
    }

    pub fn edit_text(&mut self, text: TextView, new_text: String) {
        self.changed = true;

        self.texts.get_mut(text.id).unwrap().text = new_text;
        self.texts.get_mut(text.id).unwrap().generate_text_mesh(&self.atlas_coords);
    }

    // Totals the vertices if any of the text has changed
    pub fn total(&mut self, device: &Device) {

        if self.changed {
            self.changed = false;

            self.total_vertices.clear();
            self.total_indices.clear();

            for text in &mut self.texts {
                self.total_vertices.append(&mut text.vertices);
                let starting_indices = self.total_indices.len();
                for index in &text.indices {
                    self.total_indices.push(index + starting_indices as u16);
                }
            }

            self.total_vertex_buffer = Some(device.create_buffer_mapped(self.total_vertices.len(), wgpu::BufferUsage::VERTEX)
                .fill_from_slice(self.total_vertices.as_mut_slice()));

            self.total_indices_buffer = Some(device.create_buffer_mapped(self.total_indices.len(), wgpu::BufferUsage::INDEX)
                .fill_from_slice(self.total_indices.as_mut_slice()));

        }
    }
}

struct Text {
    text: String,
    vertices: Vec<UIVertex>,
    indices: Vec<u16>,
    position: Point2<f32>,
    size: f32
}

impl Text {

    pub fn new(text: String, position: Point2<f32>, size: f32) -> Text {
        Text {
            text,
            vertices: Vec::new(),
            indices: Vec::new(),
            position,
            size
        }
    }

    fn generate_text_mesh(&mut self, fonts: &FontAtlasIndexs) {
        let mut vertices = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        let mut working_x = 0.0;

        for (i, byte) in self.text.bytes().into_iter().enumerate() {

            let (uv_top_left, uv_bottom_right) = calculate_texture_coords(byte, &fonts);

            vertices.push(UIVertex {
                position: [working_x + self.position.x, self.position.y],
                tex_coords: uv_top_left.clone()
            });
            vertices.push(UIVertex {
                position: [working_x + self.position.x, self.position.y + self.size],
                tex_coords: [uv_top_left[0], uv_bottom_right[1]]
            });
            vertices.push(UIVertex {
                position: [working_x + self.position.x + self.size, self.position.y],
                tex_coords: [uv_bottom_right[0], uv_top_left[1]]
            });
            vertices.push(UIVertex {
                position: [working_x + self.position.x + self.size, self.position.y + self.size],
                tex_coords: uv_bottom_right
            });

            let vertices_count = 4;

            indices.push(i as u16 * vertices_count);
            indices.push((i as u16 * vertices_count) + 1);
            indices.push((i as u16 * vertices_count) + 3);

            indices.push(i as u16 * vertices_count);
            indices.push((i as u16 * vertices_count) + 2);
            indices.push((i as u16 * vertices_count) + 3);

            working_x += self.size;

            // Add spacing between letters
            working_x += LETTER_SPACING;
        }

        self.vertices = vertices;
        self.indices = indices;
    }
}

pub fn calculate_texture_coords(byte: u8, fonts: &FontAtlasIndexs) -> TextureAtlasIndex {
    let mut row = (byte as f32 / FONT_TEXTURE_SIZE).floor();

    // Calculate the local offset of the character inside the character sheets
    let local_top_left = [(byte as f32 % FONT_TEXTURE_SIZE as f32) / FONT_TEXTURE_SIZE, row / FONT_TEXTURE_SIZE];
    let local_bottom_right = [local_top_left[0] + (1.0 / FONT_TEXTURE_SIZE), local_top_left[1] + (1.0 / FONT_TEXTURE_SIZE)];

    // Get the position of the character sheet inside the texture atlas
    let (character_sheet_top_left, character_sheet_bottom_right) = &fonts.ascii;

    // Calculate the uv maps from both the local and the absolute
    let uv_top_left = [
        lerp(character_sheet_top_left[0], character_sheet_bottom_right[0], local_top_left[0]),
        lerp(character_sheet_top_left[1], character_sheet_bottom_right[1], local_top_left[1])];

    let uv_bottom_right = [
        lerp(character_sheet_top_left[0], character_sheet_bottom_right[0], local_bottom_right[0]),
        lerp(character_sheet_top_left[1], character_sheet_bottom_right[1], local_bottom_right[1])];

    (uv_top_left, uv_bottom_right)
}

pub fn lerp(mut a: f32, mut b: f32, t: f32) -> f32 {
    ((b - a) * t) + a
}
