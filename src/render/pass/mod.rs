use crate::render::RenderState;
use crate::services::asset_service::AssetService;
use crate::services::chunk_service::chunk::{Chunk, Chunks};
use crate::services::chunk_service::ChunkService;
use crate::services::ui_service::UIService;
use specs::{Read, System, Write};
use wgpu::{Color, LoadOp, Operations};

pub mod buffer;
pub mod prepass;
pub mod uniforms;

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        Write<'a, RenderState>,
        Read<'a, AssetService>,
        Read<'a, ChunkService>,
        Read<'a, Chunks>,
        Read<'a, UIService>,
    );

    /// Renders all visible chunks
    fn run(
        &mut self,
        (mut render_state, asset_service, chunk_service, chunks, ui_service): Self::SystemData,
    ) {
        let frame = render_state
            .swap_chain
            .as_mut()
            .unwrap()
            .get_current_frame()
            .unwrap();

        let mut encoder = render_state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.output.view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(Color::BLACK),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: Some(
                        wgpu::RenderPassDepthStencilAttachmentDescriptor {
                            attachment: &render_state.depth_texture.1,
                            depth_ops: Some(Operations {
                                load: LoadOp::Clear(1.0),
                                store: true,
                            }),
                            stencil_ops: None,
                        },
                    ),
                });

                render_pass.set_pipeline(&render_state.render_pipeline);
                render_pass.set_bind_group(
                    0,
                    &asset_service.atlas_bind_group.as_ref().unwrap().clone(),
                    &[],
                );
                render_pass.set_bind_group(1, &render_state.uniform_bind_group, &[]);

                for pos in &chunk_service.visible_chunks {
                    if let Chunk::Tangible(chunk) = chunks.0.get(pos).unwrap() {
                        let indices_buffer = chunk.indices_buffer.as_ref().unwrap();
                        let vertices_buffer = chunk.vertices_buffer.as_ref().unwrap();
                        let model_bind_group = chunk.model_bind_group.as_ref().unwrap();

                        render_pass.set_bind_group(2, model_bind_group, &[0]);
                        render_pass.set_vertex_buffer(0, vertices_buffer.slice(..));
                        render_pass.set_index_buffer(indices_buffer.slice(..));
                        render_pass.draw_indexed(0..chunk.indices_buffer_len, 0, 0..1);
                    }
                }
            }

            // Debug information

            ui_service.render(&frame, &mut encoder, &render_state.device, &asset_service);

            render_state.queue.submit(Some(encoder.finish()));
        }

        std::mem::drop(frame);
    }
}
