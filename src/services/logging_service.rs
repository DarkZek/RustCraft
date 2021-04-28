use crate::services::settings_service::SettingsService;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::render::RenderState;
use lazy_static::lazy_static;
use rc_logging::LoggingService;
use specs::{Read, System};

pub struct LoggingSystem;

impl<'a> System<'a> for LoggingSystem {
    type SystemData = (Read<'a, LoggingService>, Read<'a, RenderState>);

    fn run(&mut self, (logging_service, render_state): Self::SystemData) {
        // Frames counter is at 0 once per second
        if render_state.frames != 0 {
            return;
        }

        logging_service.flush_buffer();
    }
}
