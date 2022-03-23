use crate::elements::ElementData;
use crate::positioning::Layout;
use crate::render::pipeline::UIRenderPipeline;
use crate::render::{get_device, get_swapchain_format};
use crate::vertex::UIVertex;
use crate::{ComponentData, UIController};
use nalgebra::Vector2;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroupLayout, BindingResource, Extent3d, SamplerDescriptor, Texture, TextureViewDescriptor,
};

impl UIController {
    /// Updates a components visual elements to keep them up to date
    pub(crate) fn process_component(component: &mut ComponentData) {
        let mut data = component.data.lock().unwrap();

        // If we don't need to re-render, or render for the first time then don't bother
        if !data.rerender() && component.element_vertices_buffer.is_some() && !component.dirty {
            return;
        }

        component.objects.clear();
        for obj in data.render() {
            component.objects.push(ElementData::wrap(obj));
        }

        let mut total_vertices = Vec::new();

        for element in &component.objects {
            total_vertices.append(&mut element.data.render(data.positioning()));
        }

        let vertex_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
            label: Some("UI Data Component Mesh Data Buffer"),
            contents: &bytemuck::cast_slice(total_vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        component.element_vertices_buffer = Some(vertex_buffer);
        component.element_vertices = total_vertices.len() as u32;

        // Set dirty
        component.dirty = false;
    }
}
