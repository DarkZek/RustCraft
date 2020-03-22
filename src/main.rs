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
use std::time::{SystemTime};

extern crate zerocopy;

pub mod render;
pub mod block;
pub mod world;
pub mod client;
pub mod game;
pub mod entity;

fn main() {

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let mut render_state = RenderState::new(&window);
    let mut game_state = GameState::new();

    let mut game_changes_context = GameChangesContext::new();
    game_changes_context.update_mouse_home(window.inner_size());
    let mut changes = GameChanges::new();

    let mut last_frame_time = SystemTime::now();

    let mut fps = 0;
    let mut fps_counter_frames = 0;
    let mut fps_counter_time = SystemTime::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        render_state.resize(*physical_size);
                        game_changes_context.update_mouse_home(window.inner_size());
                        *control_flow = ControlFlow::Poll;
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        render_state.resize(**new_inner_size);
                        *control_flow = ControlFlow::Poll;
                    }
                    _ => {
                        changes.handle_event(event, &mut game_changes_context, &window);
                        *control_flow = ControlFlow::Poll;
                    },
                }
            }
            Event::MainEventsCleared => {
                // Calculate delta time
                let delta_time = last_frame_time.elapsed().unwrap();
                last_frame_time = SystemTime::now();

                // Update fps counter
                if fps_counter_time.elapsed().unwrap().as_secs() > 0 {
                    fps = fps_counter_frames;
                    fps_counter_frames = 0;
                    fps_counter_time = SystemTime::now();
                }

                fps_counter_frames += 1;

                game_state.frame(&mut render_state, &changes, delta_time.as_secs_f64());
                changes = GameChanges::new();
                render_state.render();

                *control_flow = ControlFlow::Poll;
            }
            _ => *control_flow = ControlFlow::Poll
        }
    });

}
