use crate::render::RenderState;
use crate::services::chunk_service::ChunkService;
use specs::{System, Read, Write};
use crate::services::asset_service::AssetService;
use crate::services::ui_service::UIService;

pub mod uniforms;
pub mod prepass;

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {

    type SystemData = (Write<'a, RenderState>,
                        Read<'a, AssetService>,
                        Read<'a, ChunkService>,
                        Read<'a, UIService>);

    /// Renders all visible chunks
    fn run(&mut self, (mut render_state, asset_service, chunk_service, ui_service): Self::SystemData) {

        let frame = render_state.swap_chain.as_mut().unwrap().get_next_texture().unwrap();

        let mut encoder = render_state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
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
                    }],
                    depth_stencil_attachment: Some(
                        wgpu::RenderPassDepthStencilAttachmentDescriptor {
                            attachment: &render_state.depth_texture.1,
                            depth_load_op: wgpu::LoadOp::Clear,
                            depth_store_op: wgpu::StoreOp::Store,
                            clear_depth: 1.0,
                            stencil_load_op: wgpu::LoadOp::Load,
                            stencil_store_op: wgpu::StoreOp::Store,
                            clear_stencil: 0,
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
                    let chunk = chunk_service.chunks.get(pos).unwrap();

                    let indices_buffer = chunk.indices_buffer.as_ref().unwrap();
                    let vertices_buffer = chunk.vertices_buffer.as_ref().unwrap();
                    let model_bind_group = chunk.model_bind_group.as_ref().unwrap();

                    render_pass.set_bind_group(2, model_bind_group, &[0]);
                    render_pass.set_vertex_buffer(0, &vertices_buffer, 0, 0);
                    render_pass.set_index_buffer(indices_buffer, 0, 0);
                    render_pass.draw_indexed(0..chunk.indices_buffer_len, 0, 0..1);
                }
            }

            // Debug information

            ui_service.render(&frame, &mut encoder, &render_state.device, &asset_service);

            render_state.queue.submit(&[encoder.finish()]);
        }

        std::mem::drop(frame);
    }
}
