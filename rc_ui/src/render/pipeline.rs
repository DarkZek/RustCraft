use crate::positioning::{Layout, LayoutScheme};
use crate::render::combine::combine_render_pipeline;
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
    pub fn new(
        component_projection_bind_group_layout: &BindGroupLayout,
        size: Extent3d,
    ) -> UIRenderPipeline {
        let (projection_buffer, projection_bind_group, projection_bind_group_layout) =
            UIRenderPipeline::setup_ui_projection_matrix(size);

        let (combine_render_pipeline, combine_image_bind_group_layout) = combine_render_pipeline(
            &projection_bind_group_layout,
            component_projection_bind_group_layout,
        );

        let sampler = get_device().create_sampler(&SamplerDescriptor::default());

        let layout = Layout::new(
            Vector2::new(size.width as f32, size.height as f32),
            Vector2::zeros(),
            LayoutScheme::TopLeft,
            0.0,
        );

        log!("Setup up UI pipeline");

        UIRenderPipeline {
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
                continue;
            }

            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("UI Render Component Pass"),
                color_attachments: &[RenderPassColorAttachment {
                    view: &output_image_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::TRANSPARENT),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            pass.set_pipeline(&controller.pipeline.combine_render_pipeline);

            pass.set_bind_group(0, &controller.pipeline.projection_bind_group, &[]);

            pass.set_bind_group(1, &controller.bind_group, &[]);

            pass.set_bind_group(2, &component_data.projection_bind_group, &[]);

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
    }
}
