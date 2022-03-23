use nalgebra::Vector2;
use rc_ui::atlas::TextureAtlasIndex;
use rc_ui::component::UIComponent;
use rc_ui::elements::image::UIImage;

use rc_ui::elements::UIElement;
use rc_ui::positioning::{Layout, LayoutScheme};

const BORDER_WIDTH: f32 = 1.0;
const SLOT_COUNT: u8 = 9;
const SLOT_SIZE: f32 = 70.0;

pub struct InventoryBarComponent {
    layout: Layout,
    hotbar: TextureAtlasIndex,
    selected_slot: TextureAtlasIndex,
    slot: u8,
}

impl InventoryBarComponent {
    pub fn new(gui_image: TextureAtlasIndex) -> InventoryBarComponent {
        // Texture is 362x42, starting at 1,1
        let hotbar = gui_image.sub_index(&TextureAtlasIndex::new(
            1.0 / 512.0,
            363.0 / 512.0,
            1.0 / 512.0,
            43.0 / 512.0,
        ));

        // Texture is 46x46, starting at 1,45
        let selected_slot = gui_image.sub_index(&TextureAtlasIndex::new(
            1.0 / 512.0,
            46.0 / 512.0,
            45.0 / 512.0,
            (45.0 + 46.0) / 512.0,
        ));

        let width = (SLOT_COUNT as f32 * SLOT_SIZE) + (BORDER_WIDTH * 2.0);

        InventoryBarComponent {
            layout: Layout::new(
                Vector2::new(width, SLOT_SIZE + (BORDER_WIDTH * 2.0)),
                Vector2::new(0.0, 0.0),
                LayoutScheme::Bottom,
                0.0,
            ),
            hotbar,
            selected_slot,
            slot: 0,
        }
    }
}

impl UIComponent for InventoryBarComponent {
    fn render(&mut self) -> Vec<Box<dyn UIElement + Send + Sync + 'static>> {
        vec![
            UIImage::new(
                Layout {
                    size: Vector2::new(self.layout.size.x, self.layout.size.y),
                    offset: Vector2::new(0.0, 0.0),
                    scheme: LayoutScheme::Bottom,
                    padding: 0.0,
                },
                self.hotbar,
            ),
            UIImage::new(
                Layout {
                    size: Vector2::new(
                        SLOT_SIZE + (BORDER_WIDTH * 2.0),
                        SLOT_SIZE + (BORDER_WIDTH * 2.0),
                    ),
                    offset: Vector2::new(self.slot as f32 * SLOT_SIZE, 0.0),
                    scheme: LayoutScheme::Bottom,
                    padding: 0.0,
                },
                self.selected_slot,
            ),
        ]
    }

    fn rerender(&self) -> bool {
        false
    }

    fn positioning(&self) -> &Layout {
        &self.layout
    }

    fn resized(&mut self) {}

    fn visible(&self) -> bool {
        true
    }
}
