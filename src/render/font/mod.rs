use std::fs::File;
use std::io::Read;
use wgpu::{Device, TextureFormat};

pub struct Font {
}

impl Font {
    pub fn from_path(path: &str) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut file = File::open(format!("{}{}", path, "/default.ttf")).expect(&format!("Font not found! {}{}", path, "/default.ttf"));
        file.read_to_end(&mut buf);
        buf
    }
}