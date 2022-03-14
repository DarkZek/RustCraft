use crate::block::blocks::BlockStates;
use crate::entity::player::{PlayerEntity, PlayerEntityCameraSyncSystem};
use crate::game::game_state::{GameState, PlayerActionsSystem, ProgramState};
use crate::game::physics::interpolator::{PhysicsInterpolationFactor, PhysicsInterpolationSystem};
use crate::game::physics::player::PlayerMovementSystem;
use crate::game::physics::{Physics, PhysicsObject, PhysicsProcessingSystem};
use crate::game::systems::DeltaTime;
use crate::render::camera::Camera;
use crate::render::effects::EffectPasses;
use crate::render::pass::outline::BoxOutline;
use crate::render::pass::prepass::{PostFrame, PreFrame};
use crate::render::pass::RenderSystem;
use crate::render::RenderState;
use crate::services::asset_service::AssetService;
use crate::services::chunk_service::chunk::{ChunkData, ChunkEntityLookup};
use crate::services::chunk_service::frustum_culling::FrustumCullingSystem;
use crate::services::chunk_service::mesh::rerendering::{
    ChunkRerenderSystem, RerenderChunkFlag, UpdateChunkGraphics,
};
use crate::services::chunk_service::mesh::update::ChunkMeshUpdateSystem;
use crate::services::input_service::input::InputState;
use crate::services::logging_service::LoggingSystem;
use crate::services::networking_service::system::NetworkingSyncSystem;
use crate::services::networking_service::NetworkingService;
use crate::services::settings_service::SettingsService;
use crate::services::ui_service::components::debug_screen::DebuggingOverlaySystem;
use crate::services::ui_service::overlays::MenuOverlaySystem;
use crate::services::ui_service::UIService;
use crate::world::player_selected_block_update::PlayerSelectedBlockUpdateSystem;
use specs::{DispatcherBuilder, World, WorldExt};
use std::borrow::Borrow;
use std::ops::Deref;
use std::time::{Duration, Instant};
use winit::event::StartCause;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    event_loop::EventLoop,
};

pub mod game_state;
pub mod physics;
pub mod resources;
pub mod systems;

pub struct Game {
    start: Instant,
    universe: World,
    event_loop: Option<EventLoop<()>>,
}

impl Game {
    pub fn new() -> Game {
        if let Result::Ok(val) = std::env::var("GRAPHICS_LOGGING") {
            if val == "1" {
                env_logger::init();
            }
        }

        let start = Instant::now();

        let event_loop = EventLoop::new();

        let mut universe = World::new();

        universe.register::<PhysicsObject>();
        universe.register::<PlayerEntity>();
        universe.register::<ChunkData>();
        universe.register::<BoxOutline>();
        universe.register::<RerenderChunkFlag>();
        universe.register::<UpdateChunkGraphics>();

        let render_state = RenderState::new(&mut universe, &event_loop);
        let game_state = GameState::new(&mut universe);

        // Generate blockstates
        BlockStates::generate(
            universe.read_resource::<AssetService>().deref(),
            universe.read_resource::<SettingsService>().deref(),
        );

        let delta_time = Duration::from_millis(0);

        universe.insert(DeltaTime(delta_time.as_secs_f32()));
        universe.insert(render_state);
        universe.insert(game_state);
        universe.insert(ChunkEntityLookup::default());

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
            .with(NetworkingSyncSystem, "networking_sync", &[])
            .with(PlayerMovementSystem, "player_movement", &["pre_frame"])
            .with(PlayerActionsSystem, "player_actions", &["pre_frame"])
            .with(DebuggingOverlaySystem, "debugging_overlay", &["pre_frame"])
            .with(MenuOverlaySystem, "menu_overlays", &["pre_frame"])
            .with(
                PhysicsInterpolationSystem,
                "physics_interpolation",
                &["pre_frame"],
            )
            .with(
                PlayerSelectedBlockUpdateSystem,
                "player_selected_block_update",
                &[],
            )
            .with(
                PlayerEntityCameraSyncSystem,
                "playerentity_camera_sync",
                &[
                    "player_movement",
                    "player_actions",
                    "pre_frame",
                    "physics_interpolation",
                ],
            )
            .with(
                FrustumCullingSystem,
                "frustum_culling",
                &["pre_frame", "physics_interpolation"],
            )
            .with(
                RenderSystem,
                "render_frame",
                &[
                    "player_movement",
                    "frustum_culling",
                    "physics_interpolation",
                ],
            )
            .with(PostFrame, "post_frame", &["render_frame"])
            .with(LoggingSystem, "logging_system", &["post_frame"])
            .with(ChunkRerenderSystem, "chunk_rerendering", &["post_frame"])
            .with(
                ChunkMeshUpdateSystem,
                "chunk_mesh_updating",
                &["post_frame"],
            )
            .build();

        self.universe.insert(PhysicsInterpolationFactor::default());
        self.universe.insert(Physics::default());

        let mut physics_dispatcher = DispatcherBuilder::new()
            .with(PhysicsProcessingSystem, "physics_processing", &[])
            .build();

        let event_loop = self.event_loop.take().expect("Couldn't find event loop");

        // Physics updating needs to happen at 20 hz
        let physics_loop_length = Duration::new(0, 1_000_000_000 / 20);
        let mut time_since_physics = Instant::now();

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::NewEvents(StartCause::Init) => {
                    // Send manual resize notification because windows doesn't send one itself (sometimes).
                    let size = self
                        .universe
                        .read_resource::<RenderState>()
                        .window
                        .inner_size();
                    self.universe.write_resource::<InputState>().resized(&size);
                }
                Event::WindowEvent {
                    ref event,
                    window_id: _,
                } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        self.universe
                            .write_resource::<NetworkingService>()
                            .shutdown();
                        return;
                    }
                    WindowEvent::Resized(physical_size) => {
                        let render_state: &mut RenderState = self.universe.get_mut().unwrap();
                        render_state.resize(*physical_size);
                        self.universe
                            .write_resource::<InputState>()
                            .resized(physical_size);
                        self.universe.write_resource::<Camera>().aspect =
                            physical_size.width as f32 / physical_size.height as f32;
                        self.universe
                            .write_resource::<UIService>()
                            .update_ui_projection_matrix(
                                self.universe.read_resource::<RenderState>().borrow(),
                                physical_size,
                            );
                        self.universe
                            .write_resource::<EffectPasses>()
                            .resize_buffers();
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        let render_state: &mut RenderState = self.universe.get_mut().unwrap();
                        render_state.resize(**new_inner_size);
                    }
                    _ => {
                        self.universe
                            .write_resource::<InputState>()
                            .handle_event(event);
                    }
                },
                Event::MainEventsCleared => {
                    // Update physics in a fixed step loop
                    if self.universe.read_resource::<GameState>().state == ProgramState::InGame {
                        while time_since_physics.elapsed() > physics_loop_length {
                            time_since_physics += physics_loop_length;
                            physics_dispatcher.dispatch(&mut self.universe);
                        }

                        // Calculates a scale from 0 - 1 on the time between the previous and next physics frame
                        let time = time_since_physics.elapsed().as_nanos() as f32
                            / physics_loop_length.as_nanos() as f32;
                        self.universe
                            .write_resource::<PhysicsInterpolationFactor>()
                            .0 = time;
                    }

                    frame_dispatcher.dispatch(&mut self.universe);
                    self.universe.maintain();
                    *control_flow = ControlFlow::Poll;
                }
                _ => (),
            }
            *control_flow = ControlFlow::Poll
        });
    }
}
