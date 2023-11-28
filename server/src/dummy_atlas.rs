use rc_shared::atlas::{TextureAtlasIndex, TextureAtlasTrait};

pub struct DummyAtlas;

impl TextureAtlasTrait for DummyAtlas {
    fn exists(&self) -> bool {
        true
    }

    fn get_entry(&self, name: &str) -> Option<TextureAtlasIndex> {
        None
    }
}
