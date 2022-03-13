use crate::atlas::TextureAtlasIndex;
use crate::elements::UIElement;
use crate::fonts::{TextAlignment, FONT_TEXTURE_SIZE, LETTER_SPACING};
use crate::helpers::{draw_rect, draw_sprite};
use crate::positioning::Layout;
use crate::vertex::UIVertex;
use crate::{ATLAS_INDEXES, CHARACTER_WIDTHS};
use nalgebra::Vector2;

pub struct UIText {
    pub text: String,
    pub font_size: f32,
    pub color: [f32; 4],
    pub layout: Layout,
    pub alignment: TextAlignment,
    pub background_color: [f32; 4],
}

const UI_SCREEN_BORDER_PADDING: f32 = 8.0;

impl UIElement for UIText {
    fn render(&self, layout: &Layout) -> Vec<UIVertex> {
        let text_coords = *CHARACTER_WIDTHS.get().unwrap().read().unwrap();

        let mut vertices = Vec::new();
        let mut working_x = 0.0;

        let mut text_width = 0.0;

        // Calculate the width of the text
        for byte in self.text.bytes() {
            let proportionate_width = text_coords[byte as usize] as f32 / 255.0;
            text_width +=
                (LETTER_SPACING * self.font_size) + (self.font_size * proportionate_width);
        }

        // Align text
        match self.alignment {
            TextAlignment::Left => {}
            TextAlignment::Right => working_x = self.layout.size.x - text_width,
            TextAlignment::Center => working_x = (self.layout.size.x / 2.0) - (text_width / 2.0),
        }

        let pos = self.layout.position_object(layout);
        working_x += pos.x;

        // Displays the background
        if self.background_color[3] != 0.0 {
            let background_expansion = self.font_size / 10.0;

            vertices.append(&mut draw_rect(
                Vector2::new(
                    working_x - background_expansion,
                    pos.y - background_expansion,
                ),
                Vector2::new(
                    text_width + (background_expansion * 2.0),
                    self.font_size + (background_expansion * 2.0),
                ),
                self.background_color,
            ));
        }

        // Creates the faces for the text
        for byte in self.text.bytes().into_iter() {
            vertices.append(&mut draw_sprite(
                Vector2::new(working_x, pos.y),
                Vector2::new(self.font_size, self.font_size),
                calculate_texture_coords(byte),
                self.color,
            ));

            let proportionate_width = text_coords[byte as usize] as f32 / 255.0;

            working_x += (LETTER_SPACING * self.font_size) + (self.font_size * proportionate_width);
        }

        vertices
    }
}

pub fn calculate_texture_coords(byte: u8) -> TextureAtlasIndex {
    let fonts = *ATLAS_INDEXES
        .get()
        .unwrap()
        .read()
        .unwrap()
        .get("font/ascii")
        .unwrap();
    let row = (byte as f32 / FONT_TEXTURE_SIZE).floor();

    // Calculate the local offset of the character inside the character sheets
    let local_offset = TextureAtlasIndex::new(
        (byte as f32 % FONT_TEXTURE_SIZE as f32) / FONT_TEXTURE_SIZE,
        ((byte as f32 % FONT_TEXTURE_SIZE as f32) / FONT_TEXTURE_SIZE) + (1.0 / FONT_TEXTURE_SIZE),
        row / FONT_TEXTURE_SIZE,
        (row / FONT_TEXTURE_SIZE) + (1.0 / FONT_TEXTURE_SIZE),
    );

    // Calculate the uv maps from both the local and the absolute
    fonts.sub_index(&local_offset)
}
