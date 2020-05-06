#![feature(get_mut_unchecked)]
#![feature(fixed_size_array)]
#![feature(clamp)]

use winit::window::WindowBuilder;
use winit::event_loop::EventLoop;
use winit::event::{WindowEvent, Event};
use winit::event_loop::ControlFlow;
use crate::render::RenderState;
use crate::client::events::{GameChangesContext, GameChanges};
use crate::game::game_state::GameState;
use std::time::{SystemTime, Instant};
use systemstat::Duration;

extern crate zerocopy;
extern crate log;

#[macro_use]
pub mod services;
pub mod render;
pub mod block;
pub mod world;
pub mod client;
pub mod game;
pub mod entity;
pub mod helpers;

fn main() {

    env_logger::init();

    let start = Instant::now();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("My First World - Rustcraft")
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

    log!("Took {}s to draw first frame", start.elapsed().as_secs_f32());

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {*control_flow = ControlFlow::Exit; return;},
                    WindowEvent::Resized(physical_size) => {
                        render_state.resize(*physical_size);
                        game_changes_context.update_mouse_home(window.inner_size());
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        render_state.resize(**new_inner_size);
                    }
                    _ => {
                        changes.handle_event(event, &mut game_changes_context, &window);
                    },
                }
            }
            Event::MainEventsCleared => {
                // Calculate delta time
                last_frame_time = SystemTime::now();

                // Update fps counter
                if fps_counter_time.elapsed().unwrap().as_secs() > 0 {
                    fps = fps_counter_frames;
                    println!("FPS: {}", fps);
                    fps_counter_frames = 0;
                    fps_counter_time = SystemTime::now();
                }

                fps_counter_frames += 1;

                game_state.frame(&mut render_state, &changes, delta_time.as_secs_f64());
                changes = GameChanges::new();
                render_state.render();

                *control_flow = ControlFlow::Poll;
                delta_time = last_frame_time.elapsed().unwrap();

                render_state.services.as_ref().unwrap().logging.flush_buffer();
            }
            _ => ()
        }
        *control_flow = ControlFlow::Poll
    });

}
