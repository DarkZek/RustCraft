use crate::game::game_state::{GameState, PlayerMovementSystem};
use crate::render::RenderState;
use std::time::{Instant};
use systemstat::Duration;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    event_loop::EventLoop,
};
use specs::{World, WorldExt, DispatcherBuilder};
use crate::game::systems::DeltaTime;
use crate::render::pass::prepass::{PreFrame, PostFrame};
use crate::services::input_service::input::GameChanges;
use crate::render::pass::RenderSystem;
use crate::render::camera::Camera;
use crate::services::ui_service::fonts::system::{FontComputingSystem};
use crate::services::ui_service::fps_system::{FpsDisplayingSystem};
use crate::services::logging_service::LoggingSystem;
use crate::services::chunk_service::frustum_culling::FrustumCullingSystem;

pub mod game_state;
pub mod physics;
pub mod systems;

pub struct Game {
    start: Instant,
    universe: World,
    event_loop: Option<EventLoop<()>>
}

impl Game {
    pub fn new() -> Game {
        env_logger::init();

        let start = Instant::now();

        let event_loop = EventLoop::new();

        let mut universe = World::new();

        let render_state = RenderState::new(&mut universe, &event_loop);
        let game_state = GameState::new();

        let delta_time = Duration::from_millis(0);

        universe.insert(DeltaTime(delta_time.as_secs_f32()));
        universe.insert(render_state);
        universe.insert(game_state);

        Game {
            start,
            universe,
            event_loop: Some(event_loop)
        }
    }

    pub fn run(mut self) {

        log!(
            "Took {}s to draw first frame",
            self.start.elapsed().as_secs_f32()
        );

        let event_loop = self.event_loop.take()
            .expect("Couldn't find event loop");

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id: _,
                } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    WindowEvent::Resized(physical_size) => {
                        let render_state: &mut RenderState = self.universe.get_mut().unwrap();
                        render_state.resize(*physical_size);
                        self.universe.write_resource::<GameChanges>().resized(physical_size);
                        self.universe.write_resource::<Camera>().aspect = physical_size.width as f32 / physical_size.height as f32;
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        let render_state: &mut RenderState = self.universe.get_mut().unwrap();
                        render_state.resize(**new_inner_size);
                    }
                    _ => {
                        self.universe.write_resource::<GameChanges>().handle_event(event);
                    }
                },
                Event::MainEventsCleared => {
                    // Preframe
                    // This does things like updates the DeltaTime states and logs FPS
                    let mut pre_frame_dispatcher = DispatcherBuilder::new()
                        .with(PreFrame, "pre_frame", &[])
                        .build();
                    pre_frame_dispatcher.dispatch(&mut self.universe);

                    // Mid Frame
                    // This does stuff like frustum culling, movement, font processing and text displaying
                    let mut frame_dispatcher = DispatcherBuilder::new()
                        .with(PlayerMovementSystem, "player_movement", &[])
                        .with(FontComputingSystem, "font_computing", &[])
                        .with(FpsDisplayingSystem, "fps_displayer", &[])
                        .with(LoggingSystem, "logging_system", &[])
                        .with(FrustumCullingSystem, "frustum_culling", &[])
                        .build();
                    frame_dispatcher.dispatch(&mut self.universe);

                    // Post Frame
                    // This does stuff like rendering the frame to the screen, post processing & frame time calculations
                    let mut post_frame_dispatcher = DispatcherBuilder::new()
                        .with(RenderSystem, "render_frame", &[])
                        .with(PostFrame, "post_frame", &[])
                        .build();
                    post_frame_dispatcher.dispatch(&mut self.universe);

                    *control_flow = ControlFlow::Poll;

                    // TODO: Flush log buffer
                }
                _ => (),
            }
            *control_flow = ControlFlow::Poll
        });

        self.event_loop = Some(event_loop);
    }
}