use crate::render::RenderState;
use rc_logging::LoggingService;
use specs::{Read, System};

pub struct LoggingSystem;

impl<'a> System<'a> for LoggingSystem {
    type SystemData = (Read<'a, RenderState>);

    fn run(&mut self, (render_state): Self::SystemData) {
        // Frames counter is at 0 once per second
        if render_state.frames != 0 {
            return;
        }

        flush_log!()
    }
}
