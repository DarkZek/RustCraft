use crate::frame::DataStore;
use crate::render::render::RenderState;
use std::time::Instant;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

#[macro_use]
extern crate imgui;

pub mod frame;
pub mod model;
pub mod models;
pub mod render;
pub mod screens;

fn main() {
    env_logger::init();

    // Set up window and GPU

    let event_loop = EventLoop::new();
    let mut render = RenderState::new(&event_loop);
    let mut datastore = DataStore::new();

    let mut last_frame = Instant::now();

    let mut last_cursor = None;

    // Event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = if cfg!(feature = "metal-auto-capture") {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };
        match event {
            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { scale_factor, .. },
                ..
            } => {
                render.hidpi_factor = scale_factor;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                render.window_size = render.window.borrow().inner_size();

                render.sc_desc = wgpu::SwapChainDescriptor {
                    usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                    format: wgpu::TextureFormat::Bgra8Unorm,
                    width: render.window_size.width as u32,
                    height: render.window_size.height as u32,
                    present_mode: wgpu::PresentMode::Mailbox,
                };

                render.swap_chain = render
                    .device
                    .create_swap_chain(&render.surface, &render.sc_desc);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            }
            | Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => render.window.borrow().request_redraw(),
            Event::RedrawEventsCleared => {
                datastore.draw_frame(&mut last_frame, &mut render, &mut last_cursor);
            }
            _ => (),
        }

        render
            .platform
            .handle_event(render.imgui.io_mut(), &*render.window.borrow(), &event);
    });
}
