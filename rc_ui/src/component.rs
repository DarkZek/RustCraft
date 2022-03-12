use std::sync::{Arc, Mutex};
use wgpu::{Buffer, Texture};

pub(crate) struct ComponentData {
    pub data: Arc<Mutex<dyn UIComponent + Send + Sync>>,
    pub texture: Option<Texture>,
    pub vertices_buffer: Option<Buffer>,
    pub vertices: u32,
    pub id: usize,
}

impl ComponentData {
    pub fn wrap(data: Arc<Mutex<dyn UIComponent + Send + Sync>>) -> ComponentData {
        ComponentData {
            data,
            texture: None,
            vertices_buffer: None,
            vertices: 0,
            id: 0,
        }
    }
}

/// A component is the main building block of a UI set,
/// They are composed of smaller elements.
/// An example would be a UI Popup, or a HUD Health / Mana Pool
/// Each Component is rendered to their own image buffer
pub trait UIComponent {
    fn render(&self);

    /// Called every frame to check if we should re-render
    fn rerender(&self) -> bool;
}
