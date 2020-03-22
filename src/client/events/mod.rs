use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::client::events::key_mappings::KeyMapping;

pub mod key_mappings;
pub mod input;

// Tracks user input's since the last frame
pub struct GameChanges {
    pub movement: [i32; 2],
    pub look: [f64; 2],
    pub use_item: bool,
    pub activate_item: bool,
    pub pause: bool,
    pub jump: bool,
    pub sneak: bool,
}

pub struct GameChangesContext {
    pub mappings: KeyMapping,
    pub mouse_home: PhysicalPosition<u32>,
    pub grabbed: bool,
}

impl GameChangesContext {
    pub fn new() -> GameChangesContext {
        GameChangesContext {
            mappings: KeyMapping::default(),
            mouse_home: PhysicalPosition::new(0, 0),
            grabbed: false
        }
    }

    pub fn update_mouse_home(&mut self, size: PhysicalSize<u32>) {
        self.mouse_home = PhysicalPosition {
            x: size.width / 2,
            y: size.height / 2
        };
    }
}