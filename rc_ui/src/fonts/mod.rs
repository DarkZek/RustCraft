use image::DynamicImage;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::lazy::SyncOnceCell;
use std::sync::Mutex;
use std::sync::RwLock;

use crate::atlas::TextureAtlasIndex;
use crate::fonts::variable_width::generate_variable_width_map;

pub mod variable_width;

lazy_static! {
    pub static ref CHARACTER_WIDTHS: SyncOnceCell<RwLock<[u8; 127]>> = SyncOnceCell::new();
}

// How many letters per row inside texture
pub const FONT_TEXTURE_SIZE: f32 = 16.0;
pub const LETTER_SPACING: f32 = 0.25;

pub enum TextAlignment {
    Left,
    Right,
    Center,
}
