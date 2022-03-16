use crate::elements::{ElementData, UIElement};
use crate::positioning::Layout;
use crate::render::{get_device, get_swapchain_format};
use crate::{Extent3d, UIRenderPipeline};
use nalgebra::Vector2;
use std::sync::{Arc, Mutex};
use wgpu::{
    BindGroup, BindGroupLayout, Buffer, BufferBinding, BufferBindingType, Sampler,
    SamplerDescriptor, Texture, TextureView, TextureViewDescriptor,
};

pub(crate) struct ComponentData {
    pub id: usize,

    pub data: Arc<Mutex<dyn UIComponent + Send + Sync>>,
    pub objects: Vec<ElementData>,

    pub texture: Option<Texture>,
    pub texture_bind_group: Option<BindGroup>,
    pub texture_sampler: Sampler,
    pub texture_view: Option<TextureView>,

    /// Flag that it needs to be re-rendered
    pub dirty: bool,
    /// Flag that vertices should be re-rendered
    pub rerender: bool,

    pub projection: Buffer,
    pub projection_bind_group: BindGroup,

    pub component_vertices_buffer: Option<Buffer>,
    pub component_vertices: u32,

    pub element_vertices_buffer: Option<Buffer>,
    pub element_vertices: u32,
}

impl ComponentData {
    /// Wrap users component data in struct and create resources used for rendering the component
    pub fn wrap(data: Arc<Mutex<dyn UIComponent + Send + Sync>>) -> ComponentData {
        let size = data.lock().unwrap().positioning().size;

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

        let texture = if data.lock().unwrap().visible() {
            Some(Self::create_component_texture(size.x as u32, size.y as u32))
        } else {
            None
        };

        let texture_view = texture
            .as_ref()
            .map(|t| t.create_view(&wgpu::TextureViewDescriptor::default()));

        let texture_sampler = get_device().create_sampler(&SamplerDescriptor::default());

        ComponentData {
            data,
            objects: vec![],
            texture,
            texture_bind_group: None,
            texture_sampler,
            texture_view,
            dirty: false,
            rerender: false,
            projection,
            projection_bind_group,
            component_vertices_buffer: None,
            component_vertices: 0,
            element_vertices_buffer: None,
            id: 0,
            element_vertices: 0,
        }
    }

    /// Create a texture to cache the components view in
    pub fn create_component_texture(width: u32, height: u32) -> Texture {
        get_device().create_texture(&wgpu::TextureDescriptor {
            label: Some("UI Component texture"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: *get_swapchain_format(),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
        })
    }
}

/// A component is the main building block of a UI set,
/// They are composed of smaller elements.
/// An example would be a UI Popup, or a HUD Health / Mana Pool
/// Each Component is rendered to their own image buffer
pub trait UIComponent {
    /// Called to fetch a list of elements to render
    fn render(&mut self) -> Vec<Box<dyn UIElement + Sync + Send>>;

    /// Called every frame to check if we should re-render
    fn rerender(&self) -> bool;

    /// Called to get the current position and size, respectively
    fn positioning(&self) -> &'_ Layout;

    /// Called to get the current position and size, respectively
    fn resized(&mut self);

    /// Called to get the current position and size, respectively
    fn visible(&self) -> bool;
}
