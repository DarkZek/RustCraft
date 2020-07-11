use crate::render::RenderState;
use crate::services::ui_service::UIService;
use specs::{Read, System, Write};

pub struct FontComputingSystem;

impl<'a> System<'a> for FontComputingSystem {
    type SystemData = (Write<'a, UIService>, Read<'a, RenderState>);

    fn run(&mut self, (mut ui_service, render_state): Self::SystemData) {
        ui_service.fonts.total(&render_state.device);
    }
}
