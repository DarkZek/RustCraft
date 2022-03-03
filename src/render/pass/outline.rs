use crate::render::get_texture_format;
use crate::render::vertices::LineVertex;
use crate::services::asset_service::depth_map::DEPTH_FORMAT;
use nalgebra::Vector3;
use specs::{Component, Join, ReadStorage, VecStorage};
use std::sync::Arc;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, Buffer, BufferBindingType, BufferUsages, CommandEncoder, DepthBiasState, Device,
    LoadOp, Operations, RenderPipeline, StencilState, TextureView,
};

pub struct OutlineRenderer {
    pipeline: RenderPipeline,
    device: Arc<Device>,
}

impl OutlineRenderer {
    pub fn new(device: Arc<Device>) -> OutlineRenderer {
        let vert_shader = device.create_shader_module(&wgpu::include_spirv!(
            "../../../assets/shaders/outline_vert.spv"
        ));

        let frag_shader = device.create_shader_module(&wgpu::include_spirv!(
            "../../../assets/shaders/outline_frag.spv"
        ));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
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
                    label: Some("Unknown uniform buffer bind group layout"),
                },
            )],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Outline Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vert_shader,
                entry_point: "main",
                buffers: &[LineVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &frag_shader,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: get_texture_format(),
                    write_mask: wgpu::ColorWrites::ALL,
                    blend: None,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: StencilState {
                    front: wgpu::StencilFaceState::IGNORE,
                    back: wgpu::StencilFaceState::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        OutlineRenderer { pipeline, device }
    }

    pub fn render(
        &self,
        frame: &TextureView,
        encoder: &mut CommandEncoder,
        bind_group: &BindGroup,
        outlines: &ReadStorage<BoxOutline>,
        depth_map: &TextureView,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Outline Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &frame,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_map,
                depth_ops: Some(Operations {
                    load: LoadOp::Load,
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, bind_group, &[]);

        for outline in outlines.join() {
            render_pass.set_vertex_buffer(0, outline.buffer.as_ref().unwrap().slice(..));

            render_pass.draw(0..24, 0..1);
        }
    }
}

pub struct BoxOutline {
    pos: Vector3<f32>,
    size: Vector3<f32>,
    color: [f32; 4],
    buffer: Option<Buffer>,
}

impl Component for BoxOutline {
    type Storage = VecStorage<Self>;
}

impl BoxOutline {
    pub fn new(pos: Vector3<f32>, size: Vector3<f32>, color: [f32; 4]) -> BoxOutline {
        BoxOutline {
            pos,
            size,
            color,
            buffer: None,
        }
    }

    pub fn build(&mut self, device: &Device) {
        let vertices = vec![
            // Horizontal lines
            LineVertex {
                position: [self.pos.x, self.pos.y, self.pos.z],
                color: self.color.clone(),
            },
            LineVertex {
                position: [self.pos.x, self.pos.y + self.size.y, self.pos.z],
                color: self.color.clone(),
            },
            LineVertex {
                position: [self.pos.x, self.pos.y, self.pos.z + self.size.z],
                color: self.color.clone(),
            },
            LineVertex {
                position: [
                    self.pos.x,
                    self.pos.y + self.size.y,
                    self.pos.z + self.size.z,
                ],
                color: self.color.clone(),
            },
            LineVertex {
                position: [
                    self.pos.x + self.size.z,
                    self.pos.y,
                    self.pos.z + self.size.z,
                ],
                color: self.color.clone(),
            },
            LineVertex {
                position: [
                    self.pos.x + self.size.z,
                    self.pos.y + self.size.y,
                    self.pos.z + self.size.z,
                ],
                color: self.color.clone(),
            },
            LineVertex {
                position: [self.pos.x + self.size.x, self.pos.y, self.pos.z],
                color: self.color.clone(),
            },
            LineVertex {
                position: [
                    self.pos.x + self.size.x,
                    self.pos.y + self.size.y,
                    self.pos.z,
                ],
                color: self.color.clone(),
            },
            //
            // Bottom layer horizontal lines
            //
            LineVertex {
                position: [self.pos.x + self.size.x, self.pos.y, self.pos.z],
                color: self.color.clone(),
            },
            LineVertex {
                position: [self.pos.x, self.pos.y, self.pos.z],
                color: self.color.clone(),
            },
            LineVertex {
                position: [self.pos.x, self.pos.y, self.pos.z],
                color: self.color.clone(),
            },
            LineVertex {
                position: [self.pos.x, self.pos.y, self.pos.z + self.size.z],
                color: self.color.clone(),
            },
            LineVertex {
                position: [self.pos.x, self.pos.y, self.pos.z + self.size.z],
                color: self.color.clone(),
            },
            LineVertex {
                position: [
                    self.pos.x + self.size.x,
                    self.pos.y,
                    self.pos.z + self.size.z,
                ],
                color: self.color.clone(),
            },
            LineVertex {
                position: [self.pos.x + self.size.x, self.pos.y, self.pos.z],
                color: self.color.clone(),
            },
            LineVertex {
                position: [
                    self.pos.x + self.size.x,
                    self.pos.y,
                    self.pos.z + self.size.z,
                ],
                color: self.color.clone(),
            },
            //
            // Top layer horizontal lines
            //
            LineVertex {
                position: [
                    self.pos.x + self.size.x,
                    self.pos.y + self.size.y,
                    self.pos.z,
                ],
                color: self.color.clone(),
            },
            LineVertex {
                position: [self.pos.x, self.pos.y + self.size.y, self.pos.z],
                color: self.color.clone(),
            },
            LineVertex {
                position: [self.pos.x, self.pos.y + self.size.y, self.pos.z],
                color: self.color.clone(),
            },
            LineVertex {
                position: [
                    self.pos.x,
                    self.pos.y + self.size.y,
                    self.pos.z + self.size.z,
                ],
                color: self.color.clone(),
            },
            LineVertex {
                position: [
                    self.pos.x,
                    self.pos.y + self.size.y,
                    self.pos.z + self.size.z,
                ],
                color: self.color.clone(),
            },
            LineVertex {
                position: [
                    self.pos.x + self.size.x,
                    self.pos.y + self.size.y,
                    self.pos.z + self.size.z,
                ],
                color: self.color.clone(),
            },
            LineVertex {
                position: [
                    self.pos.x + self.size.x,
                    self.pos.y + self.size.y,
                    self.pos.z,
                ],
                color: self.color.clone(),
            },
            LineVertex {
                position: [
                    self.pos.x + self.size.x,
                    self.pos.y + self.size.y,
                    self.pos.z + self.size.z,
                ],
                color: self.color.clone(),
            },
        ];

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Outline Vertices Buffer"),
            contents: &bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });

        self.buffer = Some(buffer);
    }
}
