#![feature(get_mut_unchecked)]
#![feature(fixed_size_array)]
#![feature(clamp)]

use crate::client::events::{GameChanges, GameChangesContext};
use crate::game::game_state::GameState;
use crate::render::RenderState;
use crate::services::ui_service::ObjectAlignment;
use std::time::{Instant, SystemTime};
use systemstat::Duration;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

extern crate log;
extern crate shaderc;
extern crate zerocopy;

#[macro_use]
pub mod services;
pub mod block;
pub mod client;
pub mod entity;
pub mod game;
pub mod helpers;
pub mod render;
pub mod world;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("My First World - Rustcraft")
        .build(&event_loop)
        .unwrap();

    let mut render_state = RenderState::new(&window);
    let mut game_state = GameState::new();

    let mut game_changes_context = GameChangesContext::new();
    game_changes_context.update_mouse_home(window.inner_size());
    let mut changes = GameChanges::new();

    let mut last_frame_time = SystemTime::now();
    let mut delta_time = Duration::from_millis(0);

    let mut fps = 0;
    let mut fps_counter_frames = 0;
    let mut fps_counter_time = SystemTime::now();
    let fps_text = render_state
        .services
        .as_mut()
        .unwrap()
        .ui
        .fonts
        .create_text()
        .set_text("FPS: ?")
        .set_background(true)
        .set_size(20.0)
        .set_object_alignment(ObjectAlignment::TopLeft)
        .set_text_alignment(ObjectAlignment::TopLeft)
        .set_offset([0.0, 30.0])
        .build();

    log!(
        "Took {}s to draw first frame",
        start.elapsed().as_secs_f32()
    );

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                WindowEvent::Resized(physical_size) => {
                    render_state.resize(*physical_size);
                    game_changes_context.update_mouse_home(window.inner_size());
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    render_state.resize(**new_inner_size);
                }
                _ => {
                    changes.handle_event(event, &mut game_changes_context, &window);
                }
            },
            Event::MainEventsCleared => {
                // Calculate delta time
                delta_time = last_frame_time.elapsed().unwrap();

                // Update fps counter
                if fps_counter_time.elapsed().unwrap().as_secs() > 0 {
                    fps = fps_counter_frames;
                    render_state
                        .services
                        .as_mut()
                        .unwrap()
                        .ui
                        .fonts
                        .edit_text(&fps_text, format!("FPS: {}", fps));
                    fps_counter_frames = 0;
                    fps_counter_time = SystemTime::now();
                }

                fps_counter_frames += 1;

                game_state.frame(&mut render_state, &changes, delta_time.as_secs_f64());
                changes = GameChanges::new();
                render_state.render();

                *control_flow = ControlFlow::Poll;
                last_frame_time = SystemTime::now();

                render_state
                    .services
                    .as_ref()
                    .unwrap()
                    .logging
                    .flush_buffer();
            }
            _ => (),
        }
        *control_flow = ControlFlow::Poll
    });
}
