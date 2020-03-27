use crate::render::RenderState;
use wgpu_glyph::{Section, GlyphBrushBuilder, GlyphBrush, Scale};
use std::time::{SystemTime, Instant};
use std::ops::Add;
use std::borrow::Borrow;
use crate::render::screens::debug::draw_debug_screen;

pub mod uniforms;

impl RenderState {
    pub fn render(&mut self) {

        self.update();

        let mut swapchain = self.swap_chain.take().unwrap();
        let frame = swapchain.get_next_texture();

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            todo: 0,
        });

        {
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[
                        wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            },
                        }
                    ],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                        attachment: &self.depth_texture.1,
                        depth_load_op: wgpu::LoadOp::Clear,
                        depth_store_op: wgpu::StoreOp::Store,
                        clear_depth: 1.0,
                        stencil_load_op: wgpu::LoadOp::Load,
                        stencil_store_op: wgpu::StoreOp::Store,
                        clear_stencil: 0,
                    }),
                });

                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
                render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);

                for chunk in &self.world.chunks {
                    let indices_buffer = chunk.indices_buffer.as_ref().unwrap();
                    let vertices_buffer = chunk.vertices_buffer.as_ref().unwrap();
                    let model_bind_group = chunk.model_bind_group.as_ref().unwrap();

                    render_pass.set_bind_group(2, model_bind_group, &[0]);
                    render_pass.set_vertex_buffers(0, &[(vertices_buffer, 0)]);
                    render_pass.set_index_buffer(indices_buffer, 0);
                    render_pass.draw_indexed(0..chunk.indices_buffer_len, 0, 0..1);
                }
            }

            // Debug information
            draw_debug_screen(self, &mut encoder, &frame);

            self.queue.submit(&[
                encoder.finish()
            ]);
        }

        std::mem::drop(frame);

        self.swap_chain = Some(swapchain);
    }

    pub fn update(&mut self) {
        // Update fps
        if Instant::now().duration_since(self.fps_counter).as_secs_f32() >= 1.0 {
            self.fps = self.frames;
            self.frames = 0;
            self.fps_counter = Instant::now();
        }
        self.frames += 1;
    }
}