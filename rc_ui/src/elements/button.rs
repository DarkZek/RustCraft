use crate::atlas::TextureAtlasIndex;
use crate::elements::image::UIImage;
use crate::elements::text::UIText;
use crate::elements::UIElement;
use crate::fonts::TextAlignment;
use crate::helpers::draw_sprite;
use crate::positioning::{Layout, LayoutScheme};
use crate::vertex::UIVertex;
use crate::ATLAS_INDEXES;
use nalgebra::Vector2;

const BUTTON_TEXT_PADDING: f32 = 0.28;
// The size of the cap on the side of the button texture
const CAP_SIZE: f32 = 8.0;

// Ratio of width to height for the button texture
const BUTTON_TEXTURE_RATIO: f32 = 0.1;

pub struct UIButton {
    layout: Layout,
    color: [f32; 4],
    index: TextureAtlasIndex,
    text: UIText,
}

impl UIButton {
    pub fn new(layout: Layout, label: String) -> Box<UIButton> {
        let height = layout.size.y;

        let text = UIText {
            text: label,
            font_size: layout.size.y - ((BUTTON_TEXT_PADDING * height) * 2.0),
            color: [1.0; 4],
            layout: Layout {
                size: layout.size,
                offset: Vector2::new(
                    layout.offset.x,
                    layout.offset.y + (BUTTON_TEXT_PADDING * height),
                ),
                scheme: layout.scheme,
                padding: 0.0,
            },
            alignment: TextAlignment::Center,
            background_color: [0.0; 4],
        };

        let index = *ATLAS_INDEXES
            .get()
            .unwrap()
            .read()
            .unwrap()
            .get("gui/button_normal")
            .unwrap();

        Box::new(UIButton {
            layout,
            color: [1.0; 4],
            index,
            text,
        })
    }
}

impl UIElement for UIButton {
    fn render(&self, layout: &Layout) -> Vec<UIVertex> {
        let pos = self.layout.position_object(layout);

        let atlas_size = 1.0 / self.layout.size.y;

        // Left cap
        let mut vertices = draw_sprite(
            pos,
            Vector2::new(CAP_SIZE, self.layout.size.y),
            self.index
                .sub_index(&TextureAtlasIndex::new(0.0, atlas_size, 0.0, 1.0)),
            self.color,
        );

        // Right cap
        vertices.append(&mut draw_sprite(
            Vector2::new(pos.x + self.layout.size.x - CAP_SIZE, pos.y),
            Vector2::new(CAP_SIZE, self.layout.size.y),
            self.index
                .sub_index(&TextureAtlasIndex::new(1.0 - atlas_size, 1.0, 0.0, 1.0)),
            self.color,
        ));

        // Center bit
        vertices.append(&mut draw_sprite(
            Vector2::new(pos.x + CAP_SIZE, pos.y),
            Vector2::new(self.layout.size.x - (CAP_SIZE * 2.0), self.layout.size.y),
            self.index.sub_index(&TextureAtlasIndex::new(
                (1.0 / CAP_SIZE),
                1.0 - (1.0 / CAP_SIZE),
                0.0,
                1.0,
            )),
            self.color,
        ));

        vertices.append(&mut self.text.render(&self.layout));

        vertices
    }
}
