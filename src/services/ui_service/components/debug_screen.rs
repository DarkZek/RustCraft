use crate::entity::player::PlayerEntity;
use crate::game::physics::{Physics, PhysicsObject};
use crate::render::RenderState;
use crate::services::chunk_service::ChunkService;
use crate::services::ui_service::UIService;
use crate::VERSION;
use nalgebra::{Vector2, Vector3};
use rc_ui::component::UIComponent;

use crate::render::pass::outline::BoxOutline;
use crate::services::chunk_service::chunk::{ChunkData, ChunkEntityLookup};
use crate::services::settings_service::CHUNK_SIZE;
use crate::services::ui_service::components::UIComponents;
use rc_ui::elements::text::UIText;
use rc_ui::elements::UIElement;
use rc_ui::fonts::TextAlignment;
use rc_ui::positioning::{Layout, LayoutScheme};
use specs::{Entities, Entity, Join, Read, ReadStorage, System, Write, WriteStorage};
use wgpu::AdapterInfo;

pub struct DebugScreenComponent {
    layout: Layout,
    player_position: Vector3<f32>,
    fps: u32,
    chunks: usize,
    visible_chunks: usize,
    physics_update_rate: u32,
    device_string: String,
    light: [u8; 4],
    gpu_info: Option<AdapterInfo>,

    chunk_boundary_chunk_renderer: Option<Entity>,

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
            light: [0; 4],
            gpu_info: None,
            chunk_boundary_chunk_renderer: None,
            dirty: false,
            enabled: true,
        }
    }
}

impl UIComponent for DebugScreenComponent {
    fn render(&mut self) -> Vec<Box<dyn UIElement + Send + Sync + 'static>> {
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
                text: format!("L {:?}", self.light),
                font_size: 24.0,
                color: [1.0; 4],
                layout: Layout::new(
                    Vector2::new(500.0, 24.0),
                    Vector2::new(0.0, 150.0),
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
        ReadStorage<'a, ChunkData>,
        Read<'a, ChunkEntityLookup>,
        Read<'a, UIComponents>,
        Read<'a, RenderState>,
        Read<'a, PlayerEntity>,
        ReadStorage<'a, PhysicsObject>,
        Read<'a, Physics>,
        WriteStorage<'a, BoxOutline>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            chunk_service,
            chunks,
            chunk_lookup,
            ui_components,
            render_state,
            player,
            physics_objects,
            physics,
            mut box_outlines,
            entities,
        ): Self::SystemData,
    ) {
        let mut screen = ui_components.debug_screen_component.lock().unwrap();
        screen.dirty = false;

        if !screen.enabled {
            // Delete chunk boundary renderer
            if let Some(renderer) = screen.chunk_boundary_chunk_renderer.take() {
                entities.delete(renderer).unwrap();
            }
            return;
        }

        // If the chunk boundary renderer doesn't exist, create it
        if let None = &screen.chunk_boundary_chunk_renderer {
            let mut box_outline = BoxOutline::new(
                Vector3::new(-1.0, 69.0, 0.0),
                Vector3::new(16.0, 16.0, 16.0),
                [0.0, 0.0, 1.0, 1.0],
            );
            box_outline.build();
            let entity = entities
                .build_entity()
                .with(box_outline, &mut box_outlines)
                .build();
            screen.chunk_boundary_chunk_renderer = Some(entity);
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
        let player = physics_objects.get(player.0).unwrap();

        if screen.player_position != player.position {
            screen.player_position = player.position;

            // Fetch new light level at feet
            let chunk = chunk_lookup.map.get(&Vector3::new(
                (player.position.x / 16.0).floor() as i32,
                (player.position.y / 16.0).floor() as i32,
                (player.position.z / 16.0).floor() as i32,
            ));

            if let Some(chunk) = chunk {
                let data = chunks.get(*chunk).unwrap();
                let light = data.light_levels[((player.position[0].floor() as i32
                    % CHUNK_SIZE as i32)
                    + CHUNK_SIZE as i32) as usize
                    % CHUNK_SIZE][((player.position[1].floor() as i32
                    % CHUNK_SIZE as i32)
                    + CHUNK_SIZE as i32) as usize
                    % CHUNK_SIZE][((player.position[2].floor() as i32
                    % CHUNK_SIZE as i32)
                    + CHUNK_SIZE as i32) as usize
                    % CHUNK_SIZE];
                screen.light = light;
            } else {
                screen.light = [0; 4];
            }

            // Rebuild box
            let outline = box_outlines
                .get_mut(*screen.chunk_boundary_chunk_renderer.as_ref().unwrap())
                .unwrap();
            outline.pos = Vector3::new(
                player.position.x
                    - (((player.position.x % CHUNK_SIZE as f32) + CHUNK_SIZE as f32)
                        % CHUNK_SIZE as f32),
                player.position.y
                    - (((player.position.y % CHUNK_SIZE as f32) + CHUNK_SIZE as f32)
                        % CHUNK_SIZE as f32),
                player.position.z
                    - (((player.position.z % CHUNK_SIZE as f32) + CHUNK_SIZE as f32)
                        % CHUNK_SIZE as f32),
            );
            outline.build();

            screen.dirty = true;
        }
    }
}
