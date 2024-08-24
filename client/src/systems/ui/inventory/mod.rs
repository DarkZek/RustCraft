pub mod hotbar;

use bevy::ecs::system::Resource;
use bevy::prelude::{Entity, Handle, Image};

#[derive(Resource)]
pub struct InventoryUI {
    pub hotbar_selected_entity: Option<Entity>,
    pub hotbar_icons: Option<[Entity; 10]>,
    pub hotbar_text: Option<[Entity; 10]>,
    pub hotbar_selected_images: Option<[Handle<Image>; 10]>
}

impl Default for InventoryUI {
    fn default() -> Self {
        InventoryUI {
            hotbar_selected_entity: None,
            hotbar_icons: None,
            hotbar_text: None,
            hotbar_selected_images: None
        }
    }
}
