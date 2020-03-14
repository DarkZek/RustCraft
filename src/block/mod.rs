pub mod blocks;

pub struct Block {
    pub id: u32,
    pub name: &'static str,
    pub raw_texture_names: [&'static str; 6],
    pub texture_ids: [u32; 6],
    pub transparent: bool,
}