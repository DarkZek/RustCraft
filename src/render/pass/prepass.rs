use crate::game::systems::DeltaTime;
use crate::render::RenderState;
use crate::services::input_service::input::GameChanges;
use specs::{System, Write};
use std::time::{Instant, SystemTime};

pub struct PreFrame;

impl<'a> System<'a> for PreFrame {
    type SystemData = (Write<'a, RenderState>, Write<'a, DeltaTime>);

    fn run(&mut self, (mut render_state, mut delta_time): Self::SystemData) {
        render_state.delta_time = render_state.last_frame_time.elapsed().unwrap();

        render_state.frames += 1;

        //delta_time.0 = render_state.delta_time.as_secs_f32();
        delta_time.0 = 1.0 / 60.0;

        if render_state.frame_capture_time.elapsed().as_millis() > 1000 {
            render_state.fps = (render_state.frames as f32
                / render_state.frame_capture_time.elapsed().as_secs_f32())
                as u32;

            render_state.frame_capture_time = Instant::now();
            render_state.frames = 0;
        }
    }
}

pub struct PostFrame;

impl<'a> System<'a> for PostFrame {
    type SystemData = (Write<'a, RenderState>, Write<'a, GameChanges>);

    fn run(&mut self, (mut render_state, mut game_changes): Self::SystemData) {
        render_state.last_frame_time = SystemTime::now();

        game_changes.clear();
    }
}
