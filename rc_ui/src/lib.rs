#![feature(once_cell)]

use crate::component::{ComponentData, UIComponent};
use crate::render::pipeline::UIRenderPipeline;
use crate::render::{DEVICE, SWAPCHAIN_FORMAT};
use std::sync::{Arc, Mutex};
use wgpu::{Device, Extent3d, Texture, TextureFormat};

pub mod component;
pub mod elements;
pub mod render;
pub mod vertex;

/// The UI Controller is the main struct that holds the data for all UI data
/// It holds a UIRenderer which instructs it how to perform opertions
pub struct UIController {
    renderer: Box<dyn UIRenderer + Send + Sync>,
    components: Vec<ComponentData>,
    pipeline: UIRenderPipeline,
}

impl UIController {
    /// Creates a new renderer using the instructions from `renderer`
    pub fn new(
        renderer: Box<dyn UIRenderer + Send + Sync>,
        device: &'static Device,
        swapchain_format: &'static TextureFormat,
        size: Extent3d,
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
        }
    }

    pub fn render(&self, output_image: &Texture) {
        self.pipeline.render(self, output_image);
    }
}

pub trait UIRenderer {
    fn setup(&self) -> Vec<Arc<Mutex<dyn UIComponent + Send + Sync>>>;
}
