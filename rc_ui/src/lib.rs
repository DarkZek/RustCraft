#![feature(once_cell)]

use crate::component::{ComponentData, UIComponent};
use crate::render::pipeline::UIRenderPipeline;
use crate::render::{DEVICE, SWAPCHAIN_FORMAT};
use std::sync::{Arc, Mutex};
use wgpu::{BindGroup, CommandEncoder, Device, Extent3d, Queue, Texture, TextureFormat};

pub mod atlas;
pub mod component;
pub mod elements;
pub mod helpers;
pub mod positioning;
pub mod render;
pub mod vertex;

/// The UI Controller is the main struct that holds the data for all UI data
/// It holds a UIRenderer which instructs it how to perform opertions
pub struct UIController {
    renderer: Box<dyn UIRenderer + Send + Sync>,
    components: Vec<ComponentData>,
    pipeline: UIRenderPipeline,
    // Currently unused, will be used when atlases are split into classes
    pub atlas: Arc<Texture>,
    pub bind_group: Arc<BindGroup>,
}

impl UIController {
    /// Creates a new renderer using the instructions from `renderer`
    pub fn new(
        renderer: Box<dyn UIRenderer + Send + Sync>,
        device: &'static Device,
        swapchain_format: &'static TextureFormat,
        size: Extent3d,
        atlas: Arc<Texture>,
        bind_group: Arc<BindGroup>,
    ) -> UIController {
        DEVICE.set(device).unwrap();
        SWAPCHAIN_FORMAT.set(swapchain_format).unwrap();

        let components = renderer
            .setup()
            .into_iter()
            .map(|t| ComponentData::wrap(t))
            .collect::<Vec<ComponentData>>();

        let pipeline = UIRenderPipeline::new(size);

        UIController {
            renderer,
            components,
            pipeline,
            atlas,
            bind_group,
        }
    }

    /// Loops through all components and processes any changes made
    pub fn process(&mut self, queue: &mut Queue) {
        for component in &mut self.components {
            UIController::process_component(
                component,
                &self.pipeline.layout,
                &self.pipeline.combine_image_bind_group_layout,
            );
        }
    }

    pub fn render(&self, output_image: &Texture, encoder: &mut CommandEncoder) {
        self.pipeline.render(self, output_image, encoder);
    }
}

pub trait UIRenderer {
    fn setup(&self) -> Vec<Arc<Mutex<dyn UIComponent + Send + Sync>>>;
}
