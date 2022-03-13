use fnv::FnvBuildHasher;
use specs::World;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wgpu::{BindGroup, BindGroupLayout, Buffer, Extent3d, RenderPipeline, Texture};

use crate::helpers::AtlasIndex;
use crate::render::device::get_device;
use crate::render::get_texture_format;
use pipeline::generate_render_pipeline;
use rc_ui::atlas::TextureAtlasIndex;
use rc_ui::component::UIComponent;
use rc_ui::{UIController, UIRenderer};

use crate::services::asset_service::AssetService;
use crate::services::ui_service::components::crosshair::CrosshairComponent;
use crate::services::ui_service::components::inventory_bar::InventoryBarComponent;
use crate::services::ui_service::fonts::FontsManager;
use crate::services::ui_service::image::{ImageManager, ImageType, ImageView};
use crate::services::ServicesContext;

pub mod components;
pub mod draw;
pub mod fonts;
pub mod image;
pub mod meshdata;
pub mod pipeline;
mod projection;
pub mod render_pass;

/// Stores all info related on on screen user interfaces.
/// Contains sub services named "Managers" to manage specific tasks, like font rendering.
#[allow(dead_code)]
pub struct UIService {
    pub fonts: FontsManager,
    pub images: ImageManager,
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

        let controller = UIController::new(
            Box::new(RCRenderer::new(&assets)),
            get_device(),
            get_texture_format(),
            Extent3d {
                width: context.size.width,
                height: context.size.height,
                depth_or_array_layers: 0,
            },
            assets.atlas.as_ref().unwrap().clone(),
            assets.atlas_bind_group.as_ref().unwrap().clone(),
        );

        UIService {
            fonts,
            images,
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

pub struct RCRenderer {
    crosshair_component: Arc<Mutex<CrosshairComponent>>,
    inventory_bar_component: Arc<Mutex<InventoryBarComponent>>,
}

impl RCRenderer {
    fn new(assets: &AssetService) -> RCRenderer {
        let crosshair_component = Arc::new(Mutex::new(CrosshairComponent::new()));
        let inventory_bar_component = Arc::new(Mutex::new(InventoryBarComponent::new(
            *assets
                .atlas_index
                .as_ref()
                .unwrap()
                .get("gui/widgets")
                .unwrap(),
        )));

        RCRenderer {
            crosshair_component,
            inventory_bar_component,
        }
    }
}

impl UIRenderer for RCRenderer {
    fn setup(&self) -> Vec<Arc<Mutex<dyn UIComponent + Send + Sync>>> {
        vec![
            self.crosshair_component.clone(),
            self.inventory_bar_component.clone(),
        ]
    }
}
