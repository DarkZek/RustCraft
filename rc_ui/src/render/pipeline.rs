use crate::render::{get_device, get_swapchain_format};
use crate::vertex::UIVertex;
use crate::UIController;
use wgpu::{
    BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource,
    Buffer, Extent3d, LoadOp, Operations, RenderPassColorAttachment, RenderPipeline, ShaderStages,
    Texture, TextureSampleType, TextureViewDescriptor, TextureViewDimension, VertexState,
};

pub struct UIRenderPipeline {
    default_component_render_pipeline: RenderPipeline,

    pub projection_buffer: Buffer,
    pub projection_bind_group_layout: BindGroupLayout,
    pub projection_bind_group: BindGroup,
}

impl UIRenderPipeline {
    pub fn new(size: Extent3d) -> UIRenderPipeline {
        let vert_shader = get_device()
            .create_shader_module(&wgpu::include_spirv!("../../shaders/default_vert.spv"));

        let frag_shader = get_device()
            .create_shader_module(&wgpu::include_spirv!("../../shaders/default_frag.spv"));

        let (projection_buffer, projection_bind_group, projection_bind_group_layout) =
            UIRenderPipeline::setup_ui_projection_matrix(size);

        let render_pipeline_layout =
            get_device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("UI Default pipeline layout descriptor"),
                bind_group_layouts: &[&projection_bind_group_layout],
                push_constant_ranges: &[],
            });

        let default_component_render_pipeline =
            get_device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("UI Default pipeline"),
                layout: Option::from(&render_pipeline_layout),
                vertex: VertexState {
                    module: &vert_shader,
                    entry_point: "main",
                    buffers: &[UIVertex::desc()],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &frag_shader,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: *get_swapchain_format(),
                        write_mask: wgpu::ColorWrites::ALL,
                        blend: None,
                    }],
                }),
                multiview: None,
            });

        UIRenderPipeline {
            default_component_render_pipeline,
            projection_buffer,
            projection_bind_group_layout,
            projection_bind_group,
        }
    }

    pub fn render(&self, controller: &UIController, output_image: &Texture) {
        let mut encoder = get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("UI Render Command Encoder"),
        });

        let output_image_view = output_image.create_view(&TextureViewDescriptor::default());

        for component_data in &controller.components {
            let component = &*component_data.data.lock().unwrap();

            // If we don't need to re-render it, don't
            if !component.rerender() || component_data.texture.is_none() {
                continue;
            }

            let component_image = component_data
                .texture
                .as_ref()
                .unwrap()
                .create_view(&TextureViewDescriptor::default());

            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("UI Render Pass"),
                color_attachments: &[RenderPassColorAttachment {
                    view: &component_image,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            pass.set_pipeline(&self.default_component_render_pipeline);

            pass.set_bind_group(0, &self.projection_bind_group, &[]);

            pass.set_vertex_buffer(
                0,
                component_data.vertices_buffer.as_ref().unwrap().slice(..),
            );

            pass.draw(0..component_data.vertices, 0..1);
        }
    }
}
