#![feature(once_cell)]
#![feature(fn_traits)]

use crate::atlas::TextureAtlasIndex;
use crate::component::{ComponentData, UIComponent};
use crate::fonts::variable_width::generate_variable_width_map;
use crate::fonts::CHARACTER_WIDTHS;
use crate::render::pipeline::UIRenderPipeline;
use crate::render::{DEVICE, SWAPCHAIN_FORMAT};
use fnv::FnvBuildHasher;
use fnv::FnvHashMap;
use image::DynamicImage;
use lazy_static::lazy_static;
use specs::{Read, World};
use std::collections::HashMap;
use std::lazy::SyncOnceCell;
use std::sync::{Arc, Mutex, RwLock};
use wgpu::{BindGroup, CommandEncoder, Device, Extent3d, Queue, Texture, TextureFormat};

pub mod atlas;
pub mod component;
pub mod elements;
pub mod fonts;
pub mod helpers;
pub mod interaction;
pub mod positioning;
pub mod render;
pub mod vertex;

lazy_static! {
    pub static ref ATLAS_INDEXES: SyncOnceCell<Arc<RwLock<HashMap<String, TextureAtlasIndex, FnvBuildHasher>>>> =
        SyncOnceCell::new();
}

/// The UI Controller is the main struct that holds the data for all UI data
/// It holds a UIRenderer which instructs it how to perform operations
pub struct UIController {
    renderer: Box<dyn UIRenderer + Send + Sync>,
    components: Vec<ComponentData>,
    pipeline: UIRenderPipeline,

    // Currently unused, will be used when atlases are split into classes
    pub atlas: Arc<Texture>,
    pub atlas_image: Arc<DynamicImage>,
    pub bind_group: Arc<BindGroup>,

    pub screen_size: [u32; 2],
}

impl UIController {
    /// Creates a new renderer using the instructions from `renderer`
    pub fn new(
        renderer: Box<dyn UIRenderer + Send + Sync>,
        device: &'static Device,
        swapchain_format: &'static TextureFormat,
        size: Extent3d,
        atlas: Arc<Texture>,
        atlas_image: Arc<DynamicImage>,
        bind_group: Arc<BindGroup>,
        indexes: Arc<RwLock<HashMap<String, TextureAtlasIndex, FnvBuildHasher>>>,
    ) -> UIController {
        DEVICE.set(device).unwrap();
        SWAPCHAIN_FORMAT.set(swapchain_format).unwrap();

        let components = renderer
            .setup()
            .into_iter()
            .map(|t| ComponentData::wrap(t))
            .collect::<Vec<ComponentData>>();

        let pipeline = UIRenderPipeline::new(size);

        // Set text information
        ATLAS_INDEXES.set(indexes).unwrap();
        CHARACTER_WIDTHS
            .set(RwLock::new(generate_variable_width_map(&atlas_image)))
            .unwrap();

        UIController {
            renderer,
            components,
            pipeline,
            atlas,
            atlas_image,
            bind_group,
            screen_size: [0; 2],
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

    pub fn render(&mut self, output_image: &Texture, encoder: &mut CommandEncoder) {
        UIRenderPipeline::render(self, output_image, encoder)
    }

    pub fn clicked(&mut self, universe: &World, pressed: bool, left: bool) {
        for component in &mut self.components {
            for element in &mut component.objects {
                if element.hovered && pressed && left {
                    // Click!
                    element.data.clicked(universe);
                }
            }
        }
    }

    /// When back button pressed
    /// returns true if the event was handled by a component
    pub fn back(&mut self) -> bool {
        for component in &mut self.components {
            if component.data.lock().unwrap().back() {
                // Handled!
                return true;
            }
        }

        false
    }

    pub fn resize(&mut self, size: [u32; 2]) {
        self.screen_size = size;
    }
}

pub trait UIRenderer {
    fn setup(&self) -> Vec<Arc<Mutex<dyn UIComponent + Send + Sync>>>;
}
