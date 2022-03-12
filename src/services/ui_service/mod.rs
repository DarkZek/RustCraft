use specs::World;
use std::sync::{Arc, Mutex};
use wgpu::{BindGroup, BindGroupLayout, Buffer, RenderPipeline};

use crate::helpers::AtlasIndex;
use pipeline::generate_render_pipeline;
use rc_ui::component::UIComponent;
use rc_ui::render::{UIController, UIRenderer};

use crate::services::asset_service::AssetService;
use crate::services::ui_service::fonts::FontsManager;
use crate::services::ui_service::image::{ImageManager, ImageType, ImageView};
use crate::services::ui_service::widgets::WidgetManager;
use crate::services::ServicesContext;

pub mod crosshair;
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
    controller: UIController,
}

impl UIService {
    /// Initializes service, creating gpu bind groups, uploading fonts to the gpu etc.
    pub fn new(
        context: &mut ServicesContext,
        assets: &AssetService,
        _universe: &mut World,
    ) -> UIService {
        let fonts = FontsManager::new(&assets, context.size.clone());
        // TODO: Bind resize events
        let mut images = ImageManager::new(*context.size);
        let widget = WidgetManager::new(*context.size);

        let background_image = images
            .create_image(AtlasIndex::new_lookup("gui/options_background").lookup)
            .set_fullscreen(true)
            .set_type(ImageType::BACKGROUND(50))
            .build();

        let mut crosshair = AtlasIndex::new_lookup("gui/widgets").lookup;

        crosshair.v_max = crosshair.v_min + (crosshair.height() / 16.0);
        crosshair.u_min = crosshair.u_max - (crosshair.width() / 16.0);

        let (projection_buffer, projection_bind_group, projection_bind_group_layout) =
            UIService::setup_ui_projection_matrix(context);

        let pipeline = generate_render_pipeline(&[
            assets.atlas_bind_group_layout.as_ref().unwrap(),
            &projection_bind_group_layout,
        ]);

        let controller = UIController::new(Box::new(RCRenderer::new()));

        UIService {
            fonts,
            images,
            widget,
            pipeline,
            projection_buffer,
            projection_bind_group,
            projection_bind_group_layout,
            background_image,
            controller,
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

pub struct RCRenderer {}

impl RCRenderer {
    fn new() -> RCRenderer {
        RCRenderer {}
    }
}

impl UIRenderer for RCRenderer {
    fn setup(&self) -> Vec<Arc<Mutex<dyn UIComponent + Send + Sync>>> {
        vec![]
    }
}
