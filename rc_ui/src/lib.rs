#![feature(once_cell)]
#![feature(fn_traits)]

use crate::atlas::TextureAtlasIndex;
use crate::component::{ComponentData, UIComponent};
use crate::fonts::variable_width::generate_variable_width_map;
use crate::fonts::CHARACTER_WIDTHS;
use crate::positioning::{Layout, LayoutScheme};
use crate::render::pipeline::UIRenderPipeline;
use crate::render::{get_device, DEVICE, SWAPCHAIN_FORMAT};
use fnv::FnvBuildHasher;
use fnv::FnvHashMap;
use image::DynamicImage;
use lazy_static::lazy_static;
use nalgebra::Vector2;
use specs::{Read, World};
use std::collections::HashMap;
use std::lazy::SyncOnceCell;
use std::sync::{Arc, Mutex, RwLock};
use wgpu::{
    BindGroup, BindGroupLayout, BufferBindingType, CommandEncoder, Device, Extent3d, Queue,
    Texture, TextureFormat,
};

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
    pub component_projection_bind_group_layout: BindGroupLayout,

    pub screen_size: Extent3d,
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

        let component_projection_bind_group_layout =
            get_device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        min_binding_size: None,
                        has_dynamic_offset: false,
                    },
                    count: None,
                }],
                label: Some("UI Component Projection Matrix Bind Group Layout"),
            });

        let layout = Layout::new(
            Vector2::new(size.width as f32, size.height as f32),
            Vector2::new(0.0, 0.0),
            LayoutScheme::TopLeft,
            0.0,
        );

        let components = renderer
            .setup()
            .into_iter()
            .map(|t| ComponentData::wrap(t, &layout, &component_projection_bind_group_layout))
            .collect::<Vec<ComponentData>>();

        let pipeline = UIRenderPipeline::new(&component_projection_bind_group_layout, size);

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
            component_projection_bind_group_layout,
            screen_size: size,
        }
    }

    /// Loops through all components and processes any changes made
    pub fn process(&mut self) {
        for component in &mut self.components {
            UIController::process_component(component);
        }
    }

    pub fn render(&mut self, output_image: &Texture, encoder: &mut CommandEncoder) {
        UIRenderPipeline::render(self, output_image, encoder)
    }

    pub fn clicked(&mut self, universe: &World, pressed: bool, left: bool) {
        for component in &mut self.components {
            if !component.data.lock().unwrap().visible() {
                continue;
            }
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

    /// When the window is resized, resize all components
    pub fn resize(&mut self, queue: &mut Queue, size: [u32; 2]) {
        let mut encoder = get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("UI Projection Resize Command Encoder"),
        });

        let size = Extent3d {
            width: size[0],
            height: size[1],
            depth_or_array_layers: 1,
        };

        self.screen_size = size;
        self.pipeline
            .update_ui_projection_matrix(&mut encoder, &size);

        for component in &mut self.components {
            component.resize(&mut encoder, &self.pipeline.layout);
            component.dirty = true;
        }

        queue.submit(Some(encoder.finish()));
    }
}

pub trait UIRenderer {
    fn setup(&self) -> Vec<Arc<Mutex<dyn UIComponent + Send + Sync>>>;
}
