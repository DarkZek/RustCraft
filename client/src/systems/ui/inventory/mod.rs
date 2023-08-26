pub mod hotbar;

use bevy::ecs::system::Resource;
use bevy::prelude::Entity;

#[derive(Resource)]
pub struct InventoryUI {
    pub hotbar_selected_image: Option<Entity>,
    pub hotbar_icons: Option<[Entity; 10]>,
    pub hotbar_text: Option<[Entity; 10]>,
}

impl Default for InventoryUI {
    fn default() -> Self {
        InventoryUI {
            hotbar_selected_image: None,
            hotbar_icons: None,
            hotbar_text: None,
        }
    }
}
