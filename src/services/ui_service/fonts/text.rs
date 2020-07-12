use crate::services::asset_service::atlas::TextureAtlasIndex;
use crate::services::chunk_service::mesh::UIVertex;
use crate::services::ui_service::draw::{draw_rect, draw_sprite};
use crate::services::ui_service::fonts::{FontAtlasIndexs, FONT_TEXTURE_SIZE, LETTER_SPACING};
use crate::services::ui_service::{ObjectAlignment, Positioning};
use winit::dpi::PhysicalSize;
use crate::helpers::Lerp;

pub struct Text {
    pub text: String,
    pub vertices: Vec<UIVertex>,
    pub indices: Vec<u16>,
    pub size: f32,
    pub color: [f32; 4],
    pub alignment: ObjectAlignment,
    pub text_alignment: ObjectAlignment,
    pub offset: [f32; 2],
    pub background: bool,
    pub background_color: [f32; 4],
    pub positioning: Positioning,
    pub absolute_position: [f32; 2],
}

const UI_SCREEN_BORDER_PADDING: f32 = 8.0;

impl Text {
    pub fn generate_text_mesh(
        &mut self,
        data: (&FontAtlasIndexs, &[u8; 127]),
        screen_size: &PhysicalSize<u32>,
    ) {
        let mut alignment_x_offset = 0.0;
        let mut alignment_y_offset = 0.0;
        let mut vertices = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        let mut working_x = 0.0;
        let working_y = 0.0;

        let mut text_width = 0.0;

        // Calculate the width of the text
        for byte in self.text.bytes() {
            let proportionate_width = data.1[byte as usize] as f32 / 255.0;
            text_width += (LETTER_SPACING * self.size) + (self.size * proportionate_width);
        }

        // Deal with object alignment
        match self.text_alignment {
            ObjectAlignment::Center => {
                alignment_x_offset -= text_width / 2.0;
                alignment_y_offset += self.size / 2.0
            }
            ObjectAlignment::Left => alignment_y_offset += self.size / 2.0,
            ObjectAlignment::Right => {
                alignment_x_offset -= text_width;
                alignment_y_offset += self.size / 2.0
            }
            ObjectAlignment::Top => {
                alignment_x_offset -= text_width / 2.0;
            }
            ObjectAlignment::Bottom => {
                alignment_x_offset -= text_width / 2.0;
                alignment_y_offset -= self.size;
            }
            ObjectAlignment::TopLeft => {}
            ObjectAlignment::TopRight => {
                alignment_x_offset -= text_width;
            }
            ObjectAlignment::BottomLeft => {
                alignment_y_offset -= self.size;
            }
            ObjectAlignment::BottomRight => {
                alignment_x_offset -= text_width;
                alignment_y_offset -= self.size;
            }
        }

        // Deal with object alignment
        match self.alignment {
            ObjectAlignment::Center => {}
            ObjectAlignment::Left => {
                alignment_x_offset -= (screen_size.width / 2) as f32 - UI_SCREEN_BORDER_PADDING;
            }
            ObjectAlignment::Right => {
                alignment_x_offset += (screen_size.width / 2) as f32 - UI_SCREEN_BORDER_PADDING;
            }
            ObjectAlignment::Top => {
                alignment_y_offset -= (screen_size.height / 2) as f32 - UI_SCREEN_BORDER_PADDING;
            }
            ObjectAlignment::Bottom => {
                alignment_y_offset += (screen_size.height / 2) as f32 - UI_SCREEN_BORDER_PADDING;
            }
            ObjectAlignment::TopLeft => {
                alignment_y_offset -= (screen_size.height / 2) as f32 - UI_SCREEN_BORDER_PADDING;
                alignment_x_offset -= (screen_size.width / 2) as f32 - UI_SCREEN_BORDER_PADDING;
            }
            ObjectAlignment::TopRight => {
                alignment_y_offset -= (screen_size.height / 2) as f32 - UI_SCREEN_BORDER_PADDING;
                alignment_x_offset += (screen_size.width / 2) as f32 - UI_SCREEN_BORDER_PADDING;
            }
            ObjectAlignment::BottomLeft => {
                alignment_x_offset -= (screen_size.width / 2) as f32 - UI_SCREEN_BORDER_PADDING;
                alignment_y_offset += (screen_size.height / 2) as f32 - UI_SCREEN_BORDER_PADDING;
            }
            ObjectAlignment::BottomRight => {
                alignment_x_offset += (screen_size.width / 2) as f32 - UI_SCREEN_BORDER_PADDING;
                alignment_y_offset += (screen_size.height / 2) as f32 - UI_SCREEN_BORDER_PADDING;
            }
        }

        // Deal with positioning
        if self.positioning == Positioning::Absolute {
            alignment_x_offset = self.absolute_position[0] - (screen_size.width as f32 / 2.0);
            alignment_y_offset = self.absolute_position[1] - (screen_size.height as f32 / 2.0);
        }

        // Displays the background
        if self.background {
            let background_expansion = self.size / 10.0;

            draw_rect(
                &mut vertices,
                &mut indices,
                [
                    working_x + alignment_x_offset + self.offset[0] - background_expansion,
                    self.offset[1] + alignment_y_offset - working_y - background_expansion,
                ],
                [
                    text_width + (background_expansion * 2.0),
                    self.size + (background_expansion * 2.0),
                ],
                self.background_color,
            );
        }

        // Creates the faces for the text
        for byte in self.text.bytes().into_iter() {
            draw_sprite(
                &mut vertices,
                &mut indices,
                [
                    working_x + alignment_x_offset + self.offset[0],
                    self.offset[1] + alignment_y_offset - working_y,
                ],
                [self.size, self.size],
                calculate_texture_coords(byte, data.0),
                Some(self.color),
            );

            let proportionate_width = data.1[byte as usize] as f32 / 255.0;

            working_x += (LETTER_SPACING * self.size) + (self.size * proportionate_width);
        }

        self.vertices = vertices;
        self.indices = indices;
    }
}

pub fn calculate_texture_coords(byte: u8, fonts: &FontAtlasIndexs) -> TextureAtlasIndex {
    let row = (byte as f32 / FONT_TEXTURE_SIZE).floor();

    // Calculate the local offset of the character inside the character sheets
    let local_top_left = [
        (byte as f32 % FONT_TEXTURE_SIZE as f32) / FONT_TEXTURE_SIZE,
        row / FONT_TEXTURE_SIZE,
    ];
    let local_bottom_right = [
        local_top_left[0] + (1.0 / FONT_TEXTURE_SIZE),
        local_top_left[1] + (1.0 / FONT_TEXTURE_SIZE),
    ];

    // Get the position of the character sheet inside the texture atlas
    let (character_sheet_top_left, character_sheet_bottom_right) = &fonts.ascii;

    // Calculate the uv maps from both the local and the absolute
    let uv_top_left = [
        character_sheet_top_left[0].lerp(
            character_sheet_bottom_right[0],
            local_top_left[0]
        ),
        character_sheet_top_left[1].lerp(
            character_sheet_bottom_right[1],
            local_top_left[1],
        ),
    ];

    let uv_bottom_right = [
        character_sheet_top_left[0].lerp(
            character_sheet_bottom_right[0],
            local_bottom_right[0],
        ),
        character_sheet_top_left[1].lerp(
            character_sheet_bottom_right[1],
            local_bottom_right[1],
        ),
    ];

    (uv_top_left, uv_bottom_right)
}