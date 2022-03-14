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
    pub(crate) fn process_component(
        component: &mut ComponentData,
        parent: &Layout,
        combine_image_bind_group_layout: &BindGroupLayout,
    ) {
        let data = component.data.lock().unwrap();

        // If we don't need to re-render, or render for the first time then don't bother
        if !data.rerender()
            && component.component_vertices_buffer.is_some()
            && !component.regenerate
        {
            return;
        }

        // Set dirty
        component.dirty = true;
        component.regenerate = false;

        let layout = data.positioning();
        let position = layout.position_object(parent);

        let mut total_vertices = Vec::new();

        component.objects = data.render();

        for element in &component.objects {
            total_vertices.append(&mut element.render(data.positioning()));
        }

        let vertex_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
            label: Some("UI Data Component Mesh Data Buffer"),
            contents: &bytemuck::cast_slice(total_vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        component.element_vertices_buffer = Some(vertex_buffer);
        component.element_vertices = total_vertices.len() as u32;

        let component_vertices = vec![
            UIVertex {
                position: [position.x, position.y],
                tex_coords: [0.0; 2],
                color: [0.0; 4],
            },
            UIVertex {
                position: [position.x + layout.size.x, position.y],
                tex_coords: [1.0, 0.0],
                color: [0.0; 4],
            },
            UIVertex {
                position: [position.x, position.y + layout.size.y],
                tex_coords: [0.0, 1.0],
                color: [0.0; 4],
            },
            UIVertex {
                position: [position.x + layout.size.x, position.y + layout.size.y],
                tex_coords: [1.0, 1.0],
                color: [0.0; 4],
            },
            UIVertex {
                position: [position.x + layout.size.x, position.y],
                tex_coords: [1.0, 0.0],
                color: [0.0; 4],
            },
            UIVertex {
                position: [position.x, position.y + layout.size.y],
                tex_coords: [0.0, 1.0],
                color: [0.0; 4],
            },
        ];

        let vertex_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
            label: Some("UI Component Mesh Data Buffer"),
            contents: &bytemuck::cast_slice(&component_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        component.component_vertices_buffer = Some(vertex_buffer);
        component.component_vertices = component_vertices.len() as u32;

        component.texture_bind_group = component.texture_view.as_ref().map(|texture_view| {
            get_device().create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("UI Combine Texture Bind Group"),
                layout: &combine_image_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(&texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&component.texture_sampler),
                    },
                ],
            })
        });
    }
}
