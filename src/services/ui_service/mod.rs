use crate::services::ui_service::fonts::FontsManager;
use crate::services::ServicesContext;
use crate::services::ui_service::pipeline::generate_render_pipeline;
use wgpu::{RenderPipeline, BindGroupLayout, BindGroup, Buffer};
use crate::services::asset_service::AssetService;

pub mod fonts;
pub mod render_pass;
mod pipeline;
mod projection;

pub struct UIService {
    pub(crate) fonts: FontsManager,
    pipeline: RenderPipeline,
    projection_buffer: Buffer,
    projection_bind_group: BindGroup,
    projection_bind_group_layout: BindGroupLayout
}

impl UIService {
    pub fn new(context: &mut ServicesContext, assets: &AssetService) -> UIService {

        let (projection_buffer, projection_bind_group, projection_bind_group_layout) = UIService::setup_ui_projection_matrix(context);
        let pipeline = generate_render_pipeline(context.device,
                                                &[assets.atlas_bind_group_layout.as_ref().unwrap(),
                                                    &projection_bind_group_layout]);

        let fonts = FontsManager::new(&assets);

        UIService {
            fonts,
            pipeline,
            projection_buffer,
            projection_bind_group,
            projection_bind_group_layout
        }
    }
}
