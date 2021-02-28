use crate::services::ui_service::fonts::text::Text;
use crate::services::ui_service::fonts::{FontsManager, TextView};
use crate::services::ui_service::{ObjectAlignment, Positioning};

/// A simple builder class for creating new texts on the screen.
pub struct TextBuilder<'a> {
    text: Option<Text>,
    fonts: &'a mut FontsManager,
}

impl<'a> TextBuilder<'a> {
    pub fn new(fonts: &'a mut FontsManager) -> TextBuilder {
        TextBuilder {
            text: Some(Text {
                text: String::new(),
                vertices: Vec::new(),
                indices: Vec::new(),
                size: 16.0,
                color: [1.0, 1.0, 1.0, 1.0],
                text_alignment: ObjectAlignment::TopLeft,
                alignment: ObjectAlignment::Center,
                offset: [0.0, 0.0],
                background: true,
                background_color: [0.3, 0.3, 0.3, 0.3],
                positioning: Positioning::Relative,
                absolute_position: [0.0, 0.0],
            }),
            fonts,
        }
    }

    pub fn set_text(mut self, text: &'a str) -> TextBuilder<'a> {
        self.text.as_mut().unwrap().text = String::from(text);
        self
    }

    pub fn set_size(mut self, size: f32) -> TextBuilder<'a> {
        if size % 8.0 != 0.0 {
            log!("Size of font set to number non divisible by 8, this will result in low quality text");
        }
        self.text.as_mut().unwrap().size = size;
        self
    }

    pub fn set_color(mut self, color: [f32; 4]) -> TextBuilder<'a> {
        self.text.as_mut().unwrap().color = color;
        self
    }

    pub fn set_text_alignment(mut self, alignment: ObjectAlignment) -> TextBuilder<'a> {
        self.text.as_mut().unwrap().text_alignment = alignment;
        self
    }

    pub fn set_object_alignment(mut self, alignment: ObjectAlignment) -> TextBuilder<'a> {
        self.text.as_mut().unwrap().alignment = alignment;
        self
    }

    pub fn set_background(mut self, background: bool) -> TextBuilder<'a> {
        self.text.as_mut().unwrap().background = background;
        self
    }

    pub fn set_background_color(mut self, background_color: [f32; 4]) -> TextBuilder<'a> {
        self.text.as_mut().unwrap().background_color = background_color;
        self
    }

    pub fn set_offset(mut self, offset: [f32; 2]) -> TextBuilder<'a> {
        self.text.as_mut().unwrap().offset = offset;
        self
    }

    pub fn set_positioning(mut self, positioning: Positioning) -> TextBuilder<'a> {
        self.text.as_mut().unwrap().positioning = positioning;
        self
    }

    pub fn set_absolute_position(mut self, position: [f32; 2]) -> TextBuilder<'a> {
        self.text.as_mut().unwrap().absolute_position = position;
        self
    }

    pub fn build(mut self) -> TextView {
        self.fonts.add_text(self.text.take().unwrap())
    }
}
