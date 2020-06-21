use crate::services::asset_service::AssetService;
use crate::services::ui_service::fonts::FontsManager;
use crate::services::ui_service::pipeline::generate_render_pipeline;
use crate::services::ServicesContext;
use wgpu::{BindGroup, BindGroupLayout, Buffer, RenderPipeline};

pub mod draw;
pub mod fonts;
mod pipeline;
mod projection;
pub mod render_pass;

pub struct UIService {
    pub fonts: FontsManager,
    pipeline: RenderPipeline,
    projection_buffer: Buffer,
    projection_bind_group: BindGroup,
    projection_bind_group_layout: BindGroupLayout,
}

impl UIService {
    pub fn new(context: &mut ServicesContext, assets: &AssetService) -> UIService {
        let (projection_buffer, projection_bind_group, projection_bind_group_layout) =
            UIService::setup_ui_projection_matrix(context);
        let pipeline = generate_render_pipeline(
            context.device,
            &[
                assets.atlas_bind_group_layout.as_ref().unwrap(),
                &projection_bind_group_layout,
            ],
        );

        let fonts = FontsManager::new(&assets, context.size.clone());

        UIService {
            fonts,
            pipeline,
            projection_buffer,
            projection_bind_group,
            projection_bind_group_layout,
        }
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
