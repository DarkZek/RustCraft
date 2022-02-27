use crate::game::game_state::{GameState, ProgramState};
use crate::render::background::Background;
use crate::render::camera::Camera;
use crate::render::{get_swapchain_size, get_texture_format, RenderState};
use crate::services::asset_service::AssetService;
use crate::services::chunk_service::chunk::{ChunkData, Chunks};
use crate::services::chunk_service::ChunkService;
use crate::services::ui_service::UIService;
use specs::{Read, ReadStorage, System, Write};
use wgpu::{
    Color, Extent3d, IndexFormat, LoadOp, Operations, TextureDimension, TextureViewDescriptor,
};

pub mod buffer;
pub mod prepass;
pub mod uniforms;

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        Write<'a, RenderState>,
        Read<'a, GameState>,
        Read<'a, AssetService>,
        Read<'a, ChunkService>,
        ReadStorage<'a, ChunkData>,
        Read<'a, UIService>,
        Read<'a, Background>,
        Read<'a, Camera>,
    );

    /// Renders all visible chunks
    fn run(
        &mut self,
        (
            mut render_state,
            game_state,
            asset_service,
            chunk_service,
            chunks,
            ui_service,
            background,
            camera,
        ): Self::SystemData,
    ) {
        use specs::Join;
        let chunks = Chunks::new(chunks.join().collect::<Vec<&ChunkData>>());

        let texture = render_state.surface.get_current_texture().unwrap();
        let frame = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let bloom_texture = &render_state.post_processing.bloom.bloom_texture;
        let bloom_frame = bloom_texture.create_view(&TextureViewDescriptor::default());

        let frame_texture_descriptor = wgpu::TextureDescriptor {
            label: Some("Chunk view texture"),
            size: get_swapchain_size(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: get_texture_format(),
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
        };

        let frame_image_buffer = render_state
            .device
            .create_texture(&frame_texture_descriptor);
        let frame_image_view = frame_image_buffer.create_view(&TextureViewDescriptor::default());

        let mut encoder =
            render_state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Main Render Loop Command Encoder"),
                });

        {
            if game_state.state == ProgramState::InGame {
                background.draw(
                    &frame,
                    &mut encoder,
                    &[camera.yaw / (std::f32::consts::PI * 2.0), -camera.pitch],
                );

                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Main Render Loop Render Pass"),
                    color_attachments: &[
                        wgpu::RenderPassColorAttachment {
                            view: &frame_image_view,
                            resolve_target: None,
                            ops: Operations {
                                load: LoadOp::Load,
                                store: true,
                            },
                        },
                        wgpu::RenderPassColorAttachment {
                            view: &bloom_frame,
                            resolve_target: None,
                            ops: Operations {
                                load: LoadOp::Clear(Color::BLACK),
                                store: true,
                            },
                        },
                    ],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &render_state.depth_texture.1,
                        depth_ops: Some(Operations {
                            load: LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
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

            render_state.post_processing.bloom.create_bloom_effect(
                &render_state.device,
                &render_state.surface_desc,
                &mut encoder,
                &frame_image_buffer,
            );

            // Merge vfx buffer into main swapchain
            encoder.copy_texture_to_texture(
                frame_image_buffer.as_image_copy(),
                texture.texture.as_image_copy(),
                get_swapchain_size(),
            );

            ui_service.render(&frame, &mut encoder, &render_state.device, &asset_service);

            render_state.queue.submit(Some(encoder.finish()));

            texture.present();
        }

        std::mem::drop(frame);
    }
}
