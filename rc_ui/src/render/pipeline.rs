use crate::positioning::{Layout, LayoutScheme};
use crate::render::combine::combine_render_pipeline;
use crate::render::default::default_render_pipeline;
use crate::render::{get_device, get_swapchain_format};
use crate::vertex::UIVertex;
use crate::{ComponentData, UIController};
use nalgebra::Vector2;
use rc_logging::log;
use wgpu::{
    BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource,
    Buffer, Color, CommandEncoder, Extent3d, LoadOp, Operations, Queue, RenderPassColorAttachment,
    RenderPipeline, Sampler, SamplerDescriptor, ShaderStages, Texture, TextureSampleType,
    TextureViewDescriptor, TextureViewDimension, VertexState,
};

pub struct UIRenderPipeline {
    default_render_pipeline: RenderPipeline,
    combine_render_pipeline: RenderPipeline,
    pub combine_image_bind_group_layout: BindGroupLayout,

    pub(crate) layout: Layout,

    pub projection_buffer: Buffer,
    pub projection_bind_group_layout: BindGroupLayout,
    pub projection_bind_group: BindGroup,
    sampler: Sampler,
}

impl UIRenderPipeline {
    /// Creates render pipeline assets in GPU memory
    pub fn new(size: Extent3d) -> UIRenderPipeline {
        let (projection_buffer, projection_bind_group, projection_bind_group_layout) =
            UIRenderPipeline::setup_ui_projection_matrix(size);

        let default_render_pipeline = default_render_pipeline(&projection_bind_group_layout);
        let (combine_render_pipeline, combine_image_bind_group_layout) =
            combine_render_pipeline(&projection_bind_group_layout);

        let sampler = get_device().create_sampler(&SamplerDescriptor::default());

        let layout = Layout::new(
            Vector2::new(1280.0, 720.0),
            Vector2::zeros(),
            LayoutScheme::TopLeft,
            0.0,
        );

        log!("Setup up UI pipeline");

        UIRenderPipeline {
            default_render_pipeline,
            combine_render_pipeline,
            combine_image_bind_group_layout,
            layout,
            projection_buffer,
            projection_bind_group_layout,
            projection_bind_group,
            sampler,
        }
    }

    /// Loops through all components and renders them
    pub fn render(
        controller: &mut UIController,
        output_image: &Texture,
        encoder: &mut CommandEncoder,
    ) {
        let output_image_view = output_image.create_view(&TextureViewDescriptor::default());

        // Render components onto image
        for component_data in &mut controller.components {
            if !component_data.data.lock().unwrap().visible() {
                // Not visible, unload resources
                if component_data.texture.is_some() {
                    component_data.texture = None;
                    component_data.texture_view = None;
                    component_data.texture_bind_group = None;
                    component_data.dirty = true;
                }
                continue;
            }

            // If we don't need to re-render it, don't
            if !component_data.dirty {
                continue;
            }

            component_data.dirty = false;

            let size = component_data.data.lock().unwrap().positioning().size;

            // Ensure resources exist
            if component_data.texture.is_none() {
                component_data.texture = Some(ComponentData::create_component_texture(
                    size.x as u32,
                    size.y as u32,
                ));
                component_data.texture_view = Some(
                    component_data
                        .texture
                        .as_ref()
                        .unwrap()
                        .create_view(&TextureViewDescriptor::default()),
                );

                component_data.texture_bind_group =
                    Some(get_device().create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some("UI Combine Texture Bind Group"),
                        layout: &controller.pipeline.combine_image_bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: BindingResource::TextureView(
                                    &component_data.texture_view.as_ref().unwrap(),
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: BindingResource::Sampler(&component_data.texture_sampler),
                            },
                        ],
                    }));
            }

            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("UI Render Component Pass"),
                color_attachments: &[RenderPassColorAttachment {
                    view: &component_data.texture_view.as_ref().unwrap(),
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::TRANSPARENT),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            pass.set_pipeline(&controller.pipeline.default_render_pipeline);

            pass.set_bind_group(0, &component_data.projection_bind_group, &[]);

            pass.set_bind_group(1, &controller.bind_group, &[]);

            pass.set_vertex_buffer(
                0,
                component_data
                    .element_vertices_buffer
                    .as_ref()
                    .unwrap()
                    .slice(..),
            );

            pass.draw(0..component_data.element_vertices, 0..1);
        }

        // Render component images onto swapchain
        for component_data in &controller.components {
            if !component_data.texture.is_some() {
                continue;
            }

            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("UI Render Combination Pass"),
                color_attachments: &[RenderPassColorAttachment {
                    view: &output_image_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            pass.set_pipeline(&controller.pipeline.combine_render_pipeline);

            pass.set_bind_group(0, &controller.pipeline.projection_bind_group, &[]);

            pass.set_bind_group(1, component_data.texture_bind_group.as_ref().unwrap(), &[]);

            pass.set_vertex_buffer(
                0,
                component_data
                    .component_vertices_buffer
                    .as_ref()
                    .unwrap()
                    .slice(..),
            );

            pass.draw(0..component_data.component_vertices, 0..1);
        }
    }
}
