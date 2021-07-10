use wgpu::{SwapChainFrame, Operations, LoadOp, RenderPipeline, VertexState, MultisampleState, BlendComponent, Device, IndexFormat, Color, CommandEncoder,  ShaderStage, PushConstantRange};
use crate::services::chunk_service::mesh::{UIVertex};
use crate::render::shaders::load_shaders;
use crate::services::ui_service::meshdata::UIMeshData;
use crate::render::TEXTURE_FORMAT;

static CLEAR_COLOR: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};

pub struct Background {
    colors: Vec<(f32, [f32; 4])>,
    render_pipeline: RenderPipeline,
    data: UIMeshData
}

impl Background {
    pub fn new(device: &Device) -> Background {

        let (vs_module, fs_module) = load_shaders(
            device,
            (
                include_bytes!("../../../../RustCraft/assets/shaders/background_vert.spv"),
                include_bytes!("../../../../RustCraft/assets/shaders/background_frag.spv"),
            ),
        );

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Background render pipeline layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[PushConstantRange {
                stages: ShaderStage::all(),
                range: 0..8
            }],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Background render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[UIVertex::desc()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                clamp_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: TEXTURE_FORMAT.get().unwrap().clone(),
                    write_mask: wgpu::ColorWrite::ALL,
                    blend: Some(wgpu::BlendState {
                        color: BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::Zero,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                }],
            }),
        });

        let top_color = [127.0 / 255.0, 172.0 / 255.0, 255.0 / 255.0, 1.0];
        let bottom_color = [170.0 / 255.0, 209.0 / 255.0, 254.0 / 255.0, 1.0];
        let grey = [0.1, 0.1, 0.1, 1.0];

        let mut background = Background {
            colors: vec![(0.0, grey.clone()), (0.4, bottom_color.clone()), (0.5, bottom_color.clone()), (0.6, top_color.clone()), (2.0, top_color.clone())],
            render_pipeline,
            data: UIMeshData::new()
        };

        background.generate_background();
        background.data.build_buf(device);

        background
    }

    pub fn generate_background(&mut self) {

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let mut last_height = -std::f32::consts::PI;
        let mut last_color = self.colors.get(0).unwrap().1.clone();

        let mut indices_index = 0;

        for (height, color) in &self.colors {

            let relative_height = (height - 0.5) * std::f32::consts::PI;

            vertices.push(UIVertex {
                position: [-std::f32::consts::PI, last_height],
                tex_coords: [0.0; 2],
                color: last_color.clone()
            });

            vertices.push(UIVertex {
                position: [std::f32::consts::PI, last_height],
                tex_coords: [0.0; 2],
                color: last_color.clone()
            });

            vertices.push(UIVertex {
                position: [-std::f32::consts::PI, relative_height],
                tex_coords: [0.0; 2],
                color: color.clone()
            });

            vertices.push(UIVertex {
                position: [std::f32::consts::PI, relative_height],
                tex_coords: [0.0; 2],
                color: color.clone()
            });

            indices.push(indices_index);
            indices.push(indices_index + 1);
            indices.push(indices_index + 2);

            indices.push(indices_index + 1);
            indices.push(indices_index + 2);
            indices.push(indices_index + 3);

            indices_index += 4;

            last_height = relative_height;
            last_color = *color;
        }

        self.data.total_vertices = vertices;
        self.data.total_indices = indices;
    }

    pub fn draw(&self, frame: &SwapChainFrame, encoder: &mut CommandEncoder, cam: &[f32; 2]) {

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Background Render Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &frame.output.view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(CLEAR_COLOR),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_push_constants(ShaderStage::all(), 0, bytemuck::cast_slice(cam));

        let vertices = self.data.total_vertex_buffer.as_ref().unwrap();
        let indices_len = self.data.total_indices.len() as u32;
        let indices = self.data.total_indices_buffer.as_ref().unwrap();

        render_pass.set_vertex_buffer(0, vertices.slice(..));
        render_pass.set_index_buffer(indices.slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..indices_len, 0, 0..1);
    }
}

impl Default for Background {
    fn default() -> Self {
        unimplemented!()
    }
}