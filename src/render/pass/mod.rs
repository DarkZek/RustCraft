use crate::game::game_state::{GameState, ProgramState};
use crate::render::RenderState;
use crate::services::asset_service::AssetService;
use crate::services::chunk_service::chunk::{ChunkData, Chunks};
use crate::services::chunk_service::ChunkService;
use crate::services::ui_service::UIService;
use specs::{Read, ReadStorage, System, Write};
use wgpu::{Color, IndexFormat, LoadOp, Operations};

pub mod buffer;
pub mod prepass;
pub mod uniforms;

pub struct RenderSystem;

static CLEAR_COLOR: Color = Color {
    r: 0.4,
    g: 0.8,
    b: 1.0,
    a: 0.0,
};

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        Write<'a, RenderState>,
        Read<'a, GameState>,
        Read<'a, AssetService>,
        Read<'a, ChunkService>,
        ReadStorage<'a, ChunkData>,
        Read<'a, UIService>,
    );

    /// Renders all visible chunks
    fn run(
        &mut self,
        (mut render_state, game_state, asset_service, chunk_service, chunks, ui_service): Self::SystemData,
    ) {
        use specs::Join;
        let chunks = Chunks::new(chunks.join().collect::<Vec<&ChunkData>>());

        let frame = render_state
            .swap_chain
            .as_mut()
            .unwrap()
            .get_current_frame()
            .unwrap();

        let mut encoder =
            render_state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Main Render Loop Command Encoder"),
                });

        {
            if game_state.state == ProgramState::IN_GAME {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Main Render Loop Render Pass"),
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.output.view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(CLEAR_COLOR),
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

                // Opaque pass
                for pos in &chunk_service.visible_chunks {
                    let chunk = chunks.get_loc(*pos).unwrap();
                    if chunk.opaque_model.indices_buffer.is_none() {
                        continue;
                    }
                    let indices_buffer = chunk.opaque_model.indices_buffer.as_ref().unwrap();
                    let vertices_buffer = chunk.opaque_model.vertices_buffer.as_ref().unwrap();
                    let model_bind_group = chunk.model_bind_group.as_ref().unwrap();

                    render_pass.set_bind_group(2, model_bind_group, &[]);
                    render_pass.set_vertex_buffer(0, vertices_buffer.slice(..));
                    render_pass.set_index_buffer(indices_buffer.slice(..), IndexFormat::Uint16);
                    render_pass.draw_indexed(0..chunk.opaque_model.indices_buffer_len, 0, 0..1);
                }

                // Transparent pass
                for i in (0..chunk_service.visible_chunks.len()).rev() {
                    let pos = chunk_service.visible_chunks.get(i).unwrap();
                    let chunk = chunks.get_loc(*pos).unwrap();
                    if chunk.translucent_model.indices_buffer.is_none() {
                        continue;
                    }
                    let indices_buffer = chunk.translucent_model.indices_buffer.as_ref().unwrap();
                    let vertices_buffer = chunk.translucent_model.vertices_buffer.as_ref().unwrap();
                    let model_bind_group = chunk.model_bind_group.as_ref().unwrap();

                    render_pass.set_bind_group(2, model_bind_group, &[]);
                    render_pass.set_vertex_buffer(0, vertices_buffer.slice(..));
                    render_pass.set_index_buffer(indices_buffer.slice(..), IndexFormat::Uint16);
                    render_pass.draw_indexed(
                        0..chunk.translucent_model.indices_buffer_len,
                        0,
                        0..1,
                    );
                }
            }

            ui_service.render(&frame, &mut encoder, &render_state.device, &asset_service);

            render_state.queue.submit(Some(encoder.finish()));
        }

        std::mem::drop(frame);
    }
}
