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
    pub(crate) fn process_component(
        component: &mut ComponentData,
        parent: &Layout,
        combine_image_bind_group_layout: &BindGroupLayout,
    ) {
        let data = component.data.lock().unwrap();
        if !data.rerender() {
            return;
        }

        // TODO: Calculate
        let layout = data.positioning();
        let position = layout.position_object(parent);
        let size = Vector2::new(100.0, 100.0);

        component.texture = Some(Self::create_component_texture(size.x as u32, size.y as u32));
        component.texture_view = Some(
            component
                .texture
                .as_ref()
                .unwrap()
                .create_view(&TextureViewDescriptor::default()),
        );

        let mut total_vertices = Vec::new();

        for element in data.render() {
            total_vertices.append(&mut element.render());
        }

        let vertex_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
            label: Some("UI Data Component Mesh Data Buffer"),
            contents: &bytemuck::cast_slice(&total_vertices),
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
                position: [position.x + size.x, position.y],
                tex_coords: [1.0, 0.0],
                color: [0.0; 4],
            },
            UIVertex {
                position: [position.x, position.y + size.x],
                tex_coords: [0.0, 1.0],
                color: [0.0; 4],
            },
            UIVertex {
                position: [position.x + size.x, position.y + size.y],
                tex_coords: [1.0, 1.0],
                color: [0.0; 4],
            },
            UIVertex {
                position: [position.x + size.x, position.y],
                tex_coords: [1.0, 0.0],
                color: [0.0; 4],
            },
            UIVertex {
                position: [position.x, position.y + size.y],
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

        if let None = component.texture_sampler {
            component.texture_sampler =
                Some(get_device().create_sampler(&SamplerDescriptor::default()));
        }

        component.texture_bind_group =
            Some(get_device().create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("UI Combine Texture Bind Group"),
                layout: &combine_image_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(
                            component.texture_view.as_ref().unwrap(),
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(
                            component.texture_sampler.as_ref().unwrap(),
                        ),
                    },
                ],
            }));
    }

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
