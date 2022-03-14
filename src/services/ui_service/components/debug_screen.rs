use crate::entity::player::PlayerEntity;
use crate::game::physics::{Physics, PhysicsObject};
use crate::render::RenderState;
use crate::services::chunk_service::ChunkService;
use crate::services::ui_service::UIService;
use crate::VERSION;
use nalgebra::{Vector2, Vector3};
use rc_ui::component::UIComponent;
use rc_ui::elements::image::UIImage;
use rc_ui::elements::rect::UIRect;
use rc_ui::elements::text::UIText;
use rc_ui::elements::UIElement;
use rc_ui::fonts::TextAlignment;
use rc_ui::positioning::{Layout, LayoutScheme};
use specs::{Join, Read, ReadStorage, System, Write};
use wgpu::AdapterInfo;

pub struct DebugScreenComponent {
    layout: Layout,
    player_position: Vector3<f32>,
    fps: u32,
    chunks: usize,
    visible_chunks: usize,
    physics_update_rate: u32,
    device_string: String,
    gpu_info: Option<AdapterInfo>,

    /// Should re-render
    dirty: bool,
    pub enabled: bool,
}

impl DebugScreenComponent {
    pub fn new() -> DebugScreenComponent {
        // Texture is 362x42, starting at 1,1
        DebugScreenComponent {
            layout: Layout::new(
                Vector2::new(1280.0, 720.0),
                Vector2::new(0.0, 0.0),
                LayoutScheme::Center,
                10.0,
            ),
            player_position: Vector3::zeros(),
            fps: 0,
            chunks: 0,
            visible_chunks: 0,
            physics_update_rate: 0,
            device_string: String::new(),
            gpu_info: None,
            dirty: false,
            enabled: true,
        }
    }
}

impl UIComponent for DebugScreenComponent {
    fn render(&self) -> Vec<Box<dyn UIElement + 'static>> {
        vec![
            Box::new(UIText {
                text: format!("Rustcraft v{} Alpha", VERSION),
                font_size: 24.0,
                color: [1.0; 4],
                layout: Layout::new(
                    Vector2::new(500.0, 24.0),
                    Vector2::new(0.0, 0.0),
                    LayoutScheme::TopLeft,
                    10.0,
                ),
                alignment: TextAlignment::Left,
                background_color: [0.0, 0.0, 0.0, 0.3],
            }),
            Box::new(UIText {
                text: format!(
                    "X: {:.1} Y: {:.1} Z: {:.1}",
                    self.player_position.x, self.player_position.y, self.player_position.z
                ),
                font_size: 24.0,
                color: [1.0; 4],
                layout: Layout::new(
                    Vector2::new(500.0, 24.0),
                    Vector2::new(0.0, 30.0),
                    LayoutScheme::TopLeft,
                    10.0,
                ),
                alignment: TextAlignment::Left,
                background_color: [0.0, 0.0, 0.0, 0.3],
            }),
            Box::new(UIText {
                text: format!("FPS {}", self.fps),
                font_size: 24.0,
                color: [1.0; 4],
                layout: Layout::new(
                    Vector2::new(500.0, 24.0),
                    Vector2::new(0.0, 60.0),
                    LayoutScheme::TopLeft,
                    10.0,
                ),
                alignment: TextAlignment::Left,
                background_color: [0.0, 0.0, 0.0, 0.3],
            }),
            Box::new(UIText {
                text: format!("{} Chunk {} V", self.chunks, self.visible_chunks),
                font_size: 24.0,
                color: [1.0; 4],
                layout: Layout::new(
                    Vector2::new(500.0, 24.0),
                    Vector2::new(0.0, 90.0),
                    LayoutScheme::TopLeft,
                    10.0,
                ),
                alignment: TextAlignment::Left,
                background_color: [0.0, 0.0, 0.0, 0.3],
            }),
            Box::new(UIText {
                text: format!("Phys/s {}", self.physics_update_rate),
                font_size: 24.0,
                color: [1.0; 4],
                layout: Layout::new(
                    Vector2::new(500.0, 24.0),
                    Vector2::new(0.0, 120.0),
                    LayoutScheme::TopLeft,
                    10.0,
                ),
                alignment: TextAlignment::Left,
                background_color: [0.0, 0.0, 0.0, 0.3],
            }),
            Box::new(UIText {
                text: if self.gpu_info.is_some() {
                    format!(
                        "{} {:?}",
                        self.gpu_info.as_ref().unwrap().name,
                        self.gpu_info.as_ref().unwrap().backend
                    )
                } else {
                    String::from("Unknown Device")
                },
                font_size: 24.0,
                color: [1.0; 4],
                layout: Layout::new(
                    Vector2::new(500.0, 24.0),
                    Vector2::new(0.0, 0.0),
                    LayoutScheme::TopRight,
                    10.0,
                ),
                alignment: TextAlignment::Right,
                background_color: [0.0, 0.0, 0.0, 0.3],
            }),
        ]
    }

    fn rerender(&self) -> bool {
        self.dirty
    }

    fn positioning(&self) -> &Layout {
        &self.layout
    }

    fn resized(&mut self) {}

    fn visible(&self) -> bool {
        self.enabled
    }
}

pub struct DebuggingOverlaySystem;

impl<'a> System<'a> for DebuggingOverlaySystem {
    type SystemData = (
        Read<'a, ChunkService>,
        Write<'a, UIService>,
        Read<'a, RenderState>,
        ReadStorage<'a, PlayerEntity>,
        ReadStorage<'a, PhysicsObject>,
        Read<'a, Physics>,
    );

    fn run(
        &mut self,
        (chunk_service, mut ui_service, render_state, player, physics_objects, physics): Self::SystemData,
    ) {
        let mut screen = ui_service.debugging_screen.lock().unwrap();

        if !screen.enabled {
            return;
        }

        // Update chunks
        if screen.visible_chunks != chunk_service.visible_chunks.len() {
            screen.visible_chunks = chunk_service.visible_chunks.len();
            screen.dirty = true;
        }
        if screen.chunks != chunk_service.viewable_chunks.len() {
            screen.chunks = chunk_service.viewable_chunks.len();
            screen.dirty = true;
        }

        if screen.fps != render_state.fps {
            screen.fps = render_state.fps;
            screen.dirty = true;
        }

        if screen.physics_update_rate != physics.updates_per_second {
            screen.physics_update_rate = physics.updates_per_second;
            screen.dirty = true;
        }

        if screen.gpu_info.is_none() {
            screen.gpu_info = Some(render_state.gpu_info.clone());
            screen.dirty = true;
        }

        // Update player pos
        let (_, player) = (&player, &physics_objects).join().last().unwrap();

        if screen.player_position != player.position {
            screen.player_position = player.position;
            screen.dirty = true;
        }
    }
}
