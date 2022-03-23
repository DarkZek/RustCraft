use crate::elements::{ElementData, UIElement};
use crate::positioning::Layout;
use crate::render::{get_device, get_swapchain_format};
use crate::{Extent3d, UIRenderPipeline};
use nalgebra::Vector2;
use std::sync::{Arc, Mutex};
use wgpu::{
    BindGroup, BindGroupLayout, Buffer, BufferBinding, BufferBindingType, CommandEncoder, Queue,
    Sampler, SamplerDescriptor, Texture, TextureView, TextureViewDescriptor,
};

pub(crate) struct ComponentData {
    pub id: usize,

    pub data: Arc<Mutex<dyn UIComponent + Send + Sync>>,
    pub objects: Vec<ElementData>,

    /// Flag that vertices should be re-generated
    pub dirty: bool,

    pub projection: Buffer,
    pub projection_bind_group: BindGroup,

    pub element_vertices_buffer: Option<Buffer>,
    pub element_vertices: u32,
}

impl ComponentData {
    /// Wrap users component data in struct and create resources used for rendering the component
    pub fn wrap(
        data: Arc<Mutex<dyn UIComponent + Send + Sync>>,
        layout: &Layout,
        component_projection_bind_group_layout: &BindGroupLayout,
    ) -> ComponentData {
        let pos = data.lock().unwrap().positioning().position_object(layout);

        let projection = UIRenderPipeline::setup_ui_projection_matrix_buffer(pos.x, pos.y);

        let projection_bind_group = get_device().create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &component_projection_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(BufferBinding {
                    buffer: &projection,
                    offset: 0,
                    size: None,
                }),
            }],
            label: Some("UI Component Projection Matrix Bind Group"),
        });

        ComponentData {
            data,
            objects: vec![],
            dirty: false,
            projection,
            projection_bind_group,
            element_vertices_buffer: None,
            id: 0,
            element_vertices: 0,
        }
    }

    /// Resizes the bounds of the component
    pub fn resize(&self, encoder: &mut CommandEncoder, layout: &Layout) {
        let pos = self
            .data
            .lock()
            .unwrap()
            .positioning()
            .position_object(layout);

        let projection = UIRenderPipeline::setup_ui_projection_matrix_buffer(pos.x, pos.y);

        encoder.copy_buffer_to_buffer(
            &projection,
            0x0,
            &self.projection,
            0x0,
            std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
        );
    }
}

/// A component is the main building block of a UI set,
/// They are composed of smaller elements.
/// An example would be a UI Popup, or a HUD Health / Mana Pool
/// Each Component is rendered to their own image buffer
pub trait UIComponent {
    /// Called to fetch a list of elements to render
    fn name(&self) -> &str;

    /// Called to fetch a list of elements to render
    fn render(&mut self) -> Vec<Box<dyn UIElement + Sync + Send>>;

    /// Called every frame to check if we should re-render
    fn rerender(&self) -> bool;

    /// Called to get the current position and size, respectively
    fn positioning(&self) -> &'_ Layout;

    /// Called to get the current position and size, respectively
    fn resized(&mut self);

    /// Called when a back navigation event is received, returns if object has handled event
    fn back(&mut self) -> bool {
        false
    }

    /// Called to get the current position and size, respectively
    fn visible(&self) -> bool;
}
