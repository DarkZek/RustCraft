
use specs::World;

use std::sync::{Arc, Mutex};
use wgpu::{BindGroup, BindGroupLayout, Buffer, Extent3d, RenderPipeline};
use winit::window::Window;

use crate::helpers::AtlasIndex;
use crate::render::device::get_device;
use crate::render::get_texture_format;
use pipeline::generate_render_pipeline;

use rc_ui::component::UIComponent;
use rc_ui::{UIController, UIRenderer};

use crate::services::asset_service::AssetService;
use crate::services::ui_service::components::crosshair::CrosshairComponent;
use crate::services::ui_service::components::debug_screen::DebugScreenComponent;
use crate::services::ui_service::components::inventory_bar::InventoryBarComponent;
use crate::services::ui_service::components::pause::PauseMenuComponent;
use crate::services::ui_service::image::{ImageManager, ImageType, ImageView};
use crate::services::ServicesContext;

pub mod components;
pub mod draw;
pub mod image;
pub mod meshdata;
pub mod overlays;
pub mod pipeline;
mod projection;
pub mod render_pass;

/// Stores all info related on on screen user interfaces.
/// Contains sub services named "Managers" to manage specific tasks, like font rendering.
#[allow(dead_code)]
pub struct UIService {
    pub images: ImageManager,
    pipeline: RenderPipeline,
    projection_buffer: Buffer,
    projection_bind_group: BindGroup,
    projection_bind_group_layout: BindGroupLayout,
    pub background_image: ImageView,
    pub controller: UIController,
    window: Arc<Window>,

    debugging_screen: Arc<Mutex<DebugScreenComponent>>,
    pause_screen: Arc<Mutex<PauseMenuComponent>>,
}

impl UIService {
    /// Initializes service, creating gpu bind groups, uploading fonts to the gpu etc.
    pub fn new(
        context: &mut ServicesContext,
        assets: &AssetService,
        _universe: &mut World,
    ) -> UIService {
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

        let renderer = RCRenderer::new(&assets);

        let debugging_screen = renderer.debug_screen_component.clone();
        let pause_screen = renderer.pause_menu_component.clone();

        let controller = UIController::new(
            Box::new(renderer),
            get_device(),
            get_texture_format(),
            Extent3d {
                width: context.size.width,
                height: context.size.height,
                depth_or_array_layers: 0,
            },
            assets.atlas.as_ref().unwrap().clone(),
            assets.atlas_image.as_ref().unwrap().clone(),
            assets.atlas_bind_group.as_ref().unwrap().clone(),
            assets.atlas_index.as_ref().unwrap().clone(),
        );

        UIService {
            images,
            pipeline,
            projection_buffer,
            projection_bind_group,
            projection_bind_group_layout,
            background_image,
            controller,
            window: context.window.clone(),
            debugging_screen,
            pause_screen,
        }
    }
}

impl Default for UIService {
    fn default() -> Self {
        unimplemented!()
    }
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
    pause_menu_component: Arc<Mutex<PauseMenuComponent>>,
    debug_screen_component: Arc<Mutex<DebugScreenComponent>>,
}

impl RCRenderer {
    fn new(assets: &AssetService) -> RCRenderer {
        let crosshair_component = Arc::new(Mutex::new(CrosshairComponent::new()));
        let inventory_bar_component = Arc::new(Mutex::new(InventoryBarComponent::new(
            *assets
                .atlas_index
                .as_ref()
                .unwrap()
                .read()
                .unwrap()
                .get("gui/widgets")
                .unwrap(),
        )));
        let pause_menu_component = Arc::new(Mutex::new(PauseMenuComponent::new()));
        let debug_screen_component = Arc::new(Mutex::new(DebugScreenComponent::new()));

        RCRenderer {
            crosshair_component,
            inventory_bar_component,
            pause_menu_component,
            debug_screen_component,
        }
    }
}

impl UIRenderer for RCRenderer {
    fn setup(&self) -> Vec<Arc<Mutex<dyn UIComponent + Send + Sync>>> {
        vec![
            self.crosshair_component.clone(),
            self.inventory_bar_component.clone(),
            self.pause_menu_component.clone(),
            self.debug_screen_component.clone(),
        ]
    }
}
