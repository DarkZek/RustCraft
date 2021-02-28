use crate::render::RenderState;
use crate::services::ui_service::fonts::TextView;
use crate::services::ui_service::{ObjectAlignment, Positioning, UIService};
use specs::{Read, System, Write};

pub struct FpsDisplayingSystem;

pub struct FpsDisplayingSystemContext {
    pub text: Option<TextView>,
    pub fps: u32,
}

impl FpsDisplayingSystemContext {
    pub fn new() -> FpsDisplayingSystemContext {
        FpsDisplayingSystemContext { text: None, fps: 0 }
    }
}

impl<'a> System<'a> for FpsDisplayingSystem {
    type SystemData = (
        Write<'a, UIService>,
        Read<'a, RenderState>,
        Write<'a, FpsDisplayingSystemContext>,
    );

    fn run(&mut self, (mut ui_service, render_state, mut fps_displayer): Self::SystemData) {
        if fps_displayer.fps == render_state.fps {
            return;
        }

        if fps_displayer.text.is_none() {
            fps_displayer.text = Some(
                ui_service
                    .fonts
                    .create_text()
                    .set_text(&format!("FPS: {}", render_state.fps))
                    .set_size(24.0)
                    .set_text_alignment(ObjectAlignment::TopLeft)
                    .set_object_alignment(ObjectAlignment::TopLeft)
                    .set_positioning(Positioning::Relative)
                    .set_background(true)
                    .set_offset([0.0, 30.0])
                    .build(),
            );
        } else {
            ui_service.fonts.edit_text(
                fps_displayer.text.as_ref().unwrap(),
                format!("FPS: {}", render_state.fps),
            );
        }

        fps_displayer.fps = render_state.fps;
    }
}

impl Default for FpsDisplayingSystemContext {
    fn default() -> Self {
        FpsDisplayingSystemContext::new()
    }
}
