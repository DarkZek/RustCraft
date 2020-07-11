use crate::entity::player::{
    PlayerEntity, PlayerEntityCameraSyncSystem, PlayerEntityColliderGeneratingSystem,
};
use crate::game::game_state::{GameState, PlayerMovementSystem};
use crate::game::physics::{PhysicsObject, PhysicsProcessingSystem};
use crate::game::systems::DeltaTime;
use crate::render::camera::Camera;
use crate::render::pass::prepass::{PostFrame, PreFrame};
use crate::render::pass::RenderSystem;
use crate::render::RenderState;
use crate::services::chunk_service::frustum_culling::FrustumCullingSystem;
use crate::services::input_service::input::GameChanges;
use crate::services::logging_service::LoggingSystem;
use crate::services::ui_service::fonts::system::FontComputingSystem;
use crate::services::ui_service::fps_system::FpsDisplayingSystem;
use crate::services::ui_service::UIService;
use specs::{DispatcherBuilder, World, WorldExt};
use std::borrow::Borrow;
use std::time::Instant;
use systemstat::Duration;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    event_loop::EventLoop,
};
use crate::game::physics::interpolator::{PhysicsInterpolationFactor, PhysicsInterpolationSystem};

pub mod game_state;
pub mod physics;
pub mod systems;

pub struct Game {
    start: Instant,
    universe: World,
    event_loop: Option<EventLoop<()>>,
}

impl Game {
    pub fn new() -> Game {
        env_logger::init();

        let start = Instant::now();

        let event_loop = EventLoop::new();

        let mut universe = World::new();

        universe.register::<PhysicsObject>();
        universe.register::<PlayerEntity>();

        let render_state = RenderState::new(&mut universe, &event_loop);
        let game_state = GameState::new(&mut universe);

        let delta_time = Duration::from_millis(0);

        universe.insert(DeltaTime(delta_time.as_secs_f32()));
        universe.insert(render_state);
        universe.insert(game_state);

        Game {
            start,
            universe,
            event_loop: Some(event_loop),
        }
    }

    pub fn run(mut self) {
        log!(
            "Took {}s to draw first frame",
            self.start.elapsed().as_secs_f32()
        );

        // This dispatcher basically has three stages
        // Preframe
        // This does things like updates the DeltaTime states and logs FPS
        // Mid Frame
        // This does stuff like frustum culling, movement, font processing and text displaying
        // Post Frame
        // This does stuff like rendering the frame to the screen, post processing & frame time calculations
        let mut frame_dispatcher = DispatcherBuilder::new()
            .with(PreFrame, "pre_frame", &[])
            .with(PlayerMovementSystem, "player_movement", &["pre_frame"])
            .with(FontComputingSystem, "font_computing", &["pre_frame"])
            .with(FpsDisplayingSystem, "fps_displayer", &["pre_frame"])
            .with(LoggingSystem, "logging_system", &["pre_frame"])
            .with(PhysicsInterpolationSystem, "physics_interpolation", &["pre_frame"])
            .with(
                PlayerEntityCameraSyncSystem,
                "playerentity_camera_sync",
                &["player_movement", "pre_frame", "physics_interpolation"],
            )
            .with(FrustumCullingSystem, "frustum_culling", &["pre_frame", "physics_interpolation"])
            .with(
                RenderSystem,
                "render_frame",
                &[
                    "player_movement",
                    "font_computing",
                    "fps_displayer",
                    "logging_system",
                    "frustum_culling",
                    "physics_interpolation"
                ],
            )
            .with(PostFrame, "post_frame", &["render_frame"])
            .build();

        self.universe.insert(PhysicsInterpolationFactor::default());
        let mut physics_dispatcher = DispatcherBuilder::new()
            .with(
                PlayerEntityColliderGeneratingSystem,
                "playerentity_collider_generation",
                &[],
            )
            .with(
                PhysicsProcessingSystem,
                "physics_processing",
                &["playerentity_collider_generation"],
            )
            .build();

        let event_loop = self.event_loop.take().expect("Couldn't find event loop");

        // Physics updating needs to happen at 20 hz
        let physics_loop_length = Duration::new(0, 1_000_000_000 / 20);
        let mut time_since_physics = Instant::now();

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
                        self.universe
                            .write_resource::<GameChanges>()
                            .resized(physical_size);
                        self.universe.write_resource::<Camera>().aspect =
                            physical_size.width as f32 / physical_size.height as f32;
                        self.universe
                            .write_resource::<UIService>()
                            .update_ui_projection_matrix(
                                self.universe.read_resource::<RenderState>().borrow(),
                                physical_size,
                            );
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        let render_state: &mut RenderState = self.universe.get_mut().unwrap();
                        render_state.resize(**new_inner_size);
                    }
                    _ => {
                        self.universe
                            .write_resource::<GameChanges>()
                            .handle_event(event);
                    }
                },
                Event::MainEventsCleared => {
                    // Update physics in a fixed step loop
                    while time_since_physics.elapsed() > physics_loop_length {
                        time_since_physics += physics_loop_length;

                        physics_dispatcher.dispatch(&mut self.universe);
                    }

                    // Calculates a scale from 0 - 1 on the time between the previous and next physics frame
                    let time = time_since_physics.elapsed().as_nanos() as f32 / physics_loop_length.as_nanos() as f32;
                    self.universe.write_resource::<PhysicsInterpolationFactor>().0 = time;

                    frame_dispatcher.dispatch(&mut self.universe);
                    *control_flow = ControlFlow::Poll;
                }
                _ => (),
            }
            *control_flow = ControlFlow::Poll
        });
    }
}
