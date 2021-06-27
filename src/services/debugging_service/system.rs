use crate::entity::player::PlayerEntity;
use crate::game::physics::PhysicsObject;
use crate::render::RenderState;
use crate::services::chunk_service::ChunkService;
use crate::services::ui_service::fonts::TextView;
use crate::services::ui_service::{ObjectAlignment, Positioning, UIService};
use crate::VERSION;
use specs::{Join, Read, ReadStorage, System, Write};
use std::collections::HashMap;

pub struct DebuggingOverlayElements {
    pub texts: HashMap<DebuggingItem, TextView>,
    pub enabled: bool,
    pub fps: u32,
}

impl Default for DebuggingOverlayElements {
    fn default() -> Self {
        DebuggingOverlayElements {
            texts: HashMap::new(),
            enabled: true,
            fps: 0,
        }
    }
}

#[derive(std::cmp::Eq, PartialEq, Hash)]
pub enum DebuggingItem {
    Version,
    Position,
    FPS,
    Chunks,
    GPUInfo
}

pub struct DebuggingOverlaySystem;

impl<'a> System<'a> for DebuggingOverlaySystem {
    type SystemData = (
        Read<'a, ChunkService>,
        Write<'a, UIService>,
        Write<'a, DebuggingOverlayElements>,
        Read<'a, RenderState>,
        ReadStorage<'a, PlayerEntity>,
        ReadStorage<'a, PhysicsObject>,
    );

    fn run(
        &mut self,
        (chunk_service, mut ui_service, mut data, render_state, player, physics_objects): Self::SystemData,
    ) {
        if !data.enabled {
            return;
        }

        if data.texts.is_empty() {
            // Create texts
            let version = ui_service
                .fonts
                .create_text()
                .set_text(&format!("Rustcraft v{} Alpha", VERSION))
                .set_size(24.0)
                .set_text_alignment(ObjectAlignment::TopLeft)
                .set_object_alignment(ObjectAlignment::TopLeft)
                .set_positioning(Positioning::Relative)
                .set_background(true)
                .set_offset([0.0, 0.0])
                .build();

            data.texts.insert(DebuggingItem::Version, version);

            let pos = ui_service
                .fonts
                .create_text()
                .set_text("X: ? Y: ? Z: ?")
                .set_size(24.0)
                .set_text_alignment(ObjectAlignment::TopLeft)
                .set_object_alignment(ObjectAlignment::TopLeft)
                .set_positioning(Positioning::Relative)
                .set_background(true)
                .set_offset([0.0, 30.0])
                .build();

            data.texts.insert(DebuggingItem::Position, pos);

            let fps = ui_service
                .fonts
                .create_text()
                .set_text("FPS: ?")
                .set_size(24.0)
                .set_text_alignment(ObjectAlignment::TopLeft)
                .set_object_alignment(ObjectAlignment::TopLeft)
                .set_positioning(Positioning::Relative)
                .set_background(true)
                .set_offset([0.0, 60.0])
                .build();

            data.texts.insert(DebuggingItem::FPS, fps);

            let chunks = ui_service
                .fonts
                .create_text()
                .set_text("Chunks: ? Total ? Visible")
                .set_size(24.0)
                .set_text_alignment(ObjectAlignment::TopLeft)
                .set_object_alignment(ObjectAlignment::TopLeft)
                .set_positioning(Positioning::Relative)
                .set_background(true)
                .set_offset([0.0, 90.0])
                .build();

            data.texts.insert(DebuggingItem::Chunks, chunks);

            let gpu = ui_service
                .fonts
                .create_text()
                .set_text(&format!("{} {:?}", render_state.gpu_info.name, render_state.gpu_info.backend))
                .set_size(24.0)
                .set_text_alignment(ObjectAlignment::TopRight)
                .set_object_alignment(ObjectAlignment::TopRight)
                .set_positioning(Positioning::Relative)
                .set_background(true)
                .set_offset([0.0, 0.0])
                .build();

            data.texts.insert(DebuggingItem::GPUInfo, gpu);
        }

        // Update chunks
        ui_service.fonts.edit_text(
            data.texts.get(&DebuggingItem::Chunks).unwrap(),
            format!(
                "{} Chunks {} V",
                chunk_service.viewable_chunks.len(),
                chunk_service.visible_chunks.len()
            ),
        );

        // Update FPS
        if data.fps != render_state.fps {
            data.fps = render_state.fps;
            ui_service.fonts.edit_text(
                data.texts.get(&DebuggingItem::FPS).unwrap(),
                format!("FPS: {}", data.fps),
            );
        }

        // Update player pos
        let (_, player) = (&player, &physics_objects).join().last().unwrap();

        ui_service.fonts.edit_text(
            data.texts.get(&DebuggingItem::Position).unwrap(),
            format!(
                "X: {:.1} Y: {:.1} Z: {:.1}",
                player.position.x, player.position.y, player.position.z
            ),
        );
    }
}
