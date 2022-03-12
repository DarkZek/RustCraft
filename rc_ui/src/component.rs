use crate::elements::UIElement;
use crate::positioning::Layout;
use crate::render::get_device;
use crate::{Extent3d, UIRenderPipeline};
use nalgebra::Vector2;
use std::sync::{Arc, Mutex};
use wgpu::{
    BindGroup, BindGroupLayout, Buffer, BufferBinding, BufferBindingType, Sampler, Texture,
    TextureView,
};

pub(crate) struct ComponentData {
    pub id: usize,
    pub data: Arc<Mutex<dyn UIComponent + Send + Sync>>,
    pub texture: Option<Texture>,
    pub texture_bind_group: Option<BindGroup>,
    pub texture_sampler: Option<Sampler>,
    pub texture_view: Option<TextureView>,

    pub projection: Buffer,
    pub projection_bind_group: BindGroup,

    pub component_vertices_buffer: Option<Buffer>,
    pub component_vertices: u32,

    pub element_vertices_buffer: Option<Buffer>,
    pub element_vertices: u32,
}

impl ComponentData {
    pub fn wrap(data: Arc<Mutex<dyn UIComponent + Send + Sync>>) -> ComponentData {
        let size = data.lock().unwrap().positioning().position;

        let projection = UIRenderPipeline::setup_ui_projection_matrix_buffer(Extent3d {
            width: size.x as u32,
            height: size.y as u32,
            depth_or_array_layers: 1,
        });

        let projection_bind_group_layout =
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

        let projection_bind_group = get_device().create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &projection_bind_group_layout,
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
            texture: None,
            texture_bind_group: None,
            texture_sampler: None,
            texture_view: None,
            projection,
            projection_bind_group,
            component_vertices_buffer: None,
            component_vertices: 0,
            element_vertices_buffer: None,
            id: 0,
            element_vertices: 0,
        }
    }
}

/// A component is the main building block of a UI set,
/// They are composed of smaller elements.
/// An example would be a UI Popup, or a HUD Health / Mana Pool
/// Each Component is rendered to their own image buffer
pub trait UIComponent {
    /// Called to fetch a list of elements to render
    fn render(&self) -> Vec<Box<dyn UIElement>>;

    /// Called every frame to check if we should re-render
    fn rerender(&self) -> bool;

    /// Called to get the current position and size, respectively
    fn positioning(&self) -> &'_ Layout;
}
