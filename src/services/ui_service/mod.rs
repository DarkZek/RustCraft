use specs::World;
use wgpu::{BindGroup, BindGroupLayout, Buffer, RenderPipeline};

use pipeline::generate_render_pipeline;

use crate::services::asset_service::AssetService;
use crate::services::ui_service::fonts::FontsManager;
use crate::services::ui_service::image::{ImageManager, ImageType, ImageView};
use crate::services::ui_service::widgets::WidgetManager;
use crate::services::ServicesContext;

pub mod draw;
pub mod fonts;
pub mod image;
pub mod meshdata;
pub mod pipeline;
mod projection;
pub mod render_pass;
pub mod widgets;

/// Stores all info related on on screen user interfaces.
/// Contains sub services named "Managers" to manage specific tasks, like font rendering.
#[allow(dead_code)]
pub struct UIService {
    pub fonts: FontsManager,
    pub images: ImageManager,
    pub widget: WidgetManager,
    pipeline: RenderPipeline,
    projection_buffer: Buffer,
    projection_bind_group: BindGroup,
    projection_bind_group_layout: BindGroupLayout,
    pub background_image: ImageView,
}

impl UIService {
    /// Initializes service, creating gpu bind groups, uploading fonts to the gpu etc.
    pub fn new(
        context: &mut ServicesContext,
        assets: &AssetService,
        universe: &mut World,
    ) -> UIService {
        let fonts = FontsManager::new(&assets, context.size.clone());
        // TODO: Bind resize events
        let mut images = ImageManager::new(*context.size);
        let widget = WidgetManager::new(*context.size);

        let background_image = images
            .create_image("gui/options_background")
            .set_fullscreen(true)
            .set_type(ImageType::BACKGROUND(50))
            .build();

        let (projection_buffer, projection_bind_group, projection_bind_group_layout) =
            UIService::setup_ui_projection_matrix(context);

        let pipeline = generate_render_pipeline(
            context.device.as_ref(),
            &[
                assets.atlas_bind_group_layout.as_ref().unwrap(),
                &projection_bind_group_layout,
            ],
        );

        UIService {
            fonts,
            images,
            widget,
            pipeline,
            projection_buffer,
            projection_bind_group,
            projection_bind_group_layout,
            background_image,
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

#[derive(PartialEq, Clone, Copy)]
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
