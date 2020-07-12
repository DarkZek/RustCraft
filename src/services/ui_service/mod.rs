use crate::services::asset_service::AssetService;
use crate::services::ui_service::fonts::FontsManager;
use crate::services::ui_service::fps_system::FpsDisplayingSystemContext;
use crate::services::ui_service::pipeline::generate_render_pipeline;
use crate::services::ServicesContext;
use specs::World;
use wgpu::{BindGroup, BindGroupLayout, Buffer, RenderPipeline};

pub mod draw;
pub mod fonts;
pub mod fps_system;
mod pipeline;
mod projection;
pub mod render_pass;

/// Stores all info related on on screen user interfaces.
/// Contains sub services named "Managers" to manage specific tasks, like font rendering.
#[allow(dead_code)]
pub struct UIService {
    pub fonts: FontsManager,
    pipeline: RenderPipeline,
    projection_buffer: Buffer,
    projection_bind_group: BindGroup,
    projection_bind_group_layout: BindGroupLayout,
}

impl UIService {
    /// Initializes service, creating gpu bind groups, uploading fonts to the gpu etc.
    pub fn new(
        context: &mut ServicesContext,
        assets: &AssetService,
        universe: &mut World,
    ) -> UIService {
        let (projection_buffer, projection_bind_group, projection_bind_group_layout) =
            UIService::setup_ui_projection_matrix(context);
        let pipeline = generate_render_pipeline(
            context.device.as_ref(),
            &[
                assets.atlas_bind_group_layout.as_ref().unwrap(),
                &projection_bind_group_layout,
            ],
        );

        let fonts = FontsManager::new(&assets, context.size.clone());

        universe.insert(FpsDisplayingSystemContext::new());

        UIService {
            fonts,
            pipeline,
            projection_buffer,
            projection_bind_group,
            projection_bind_group_layout,
        }
    }
}

impl Default for UIService {
    fn default() -> Self {
        unimplemented!()
    }
}

#[derive(PartialEq, Clone)]
pub enum TextAlignment {
    Center,
    Left,
    Right,
}

#[derive(PartialEq, Clone)]
pub enum ObjectAlignment {
    Center,
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(PartialEq, Clone)]
pub enum Positioning {
    Absolute,
    Relative,
}
