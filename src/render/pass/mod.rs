use crate::game::game_state::{GameState, ProgramState};
use crate::render::background::Background;
use crate::render::camera::Camera;
use crate::render::chunks_render_bundle::ChunksRenderBundle;
use crate::render::device::get_device;
use crate::render::effects::buffer_pool::TextureBufferPool;
use crate::render::effects::EffectPasses;
use crate::render::pass::outline::BoxOutline;
use crate::render::{get_swapchain_size, RenderState};
use crate::services::asset_service::AssetService;
use crate::services::chunk_service::chunk::{ChunkData, Chunks};
use crate::services::chunk_service::ChunkService;
use crate::services::ui_service::UIService;
use nalgebra::Vector3;
use specs::{Read, ReadStorage, System, Write};
use std::hash::Hash;
use std::mem;
use std::time::{Duration, Instant};
use wgpu::{
    Color, IndexFormat, LoadOp, Operations, RenderBundleEncoderDescriptor, TextureViewDescriptor,
};

pub mod buffer;
pub mod outline;
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
        Write<'a, UIService>,
        Read<'a, Background>,
        Read<'a, Camera>,
        ReadStorage<'a, BoxOutline>,
        Read<'a, EffectPasses>,
        Write<'a, TextureBufferPool>,
        Read<'a, ChunksRenderBundle>,
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
            mut ui_service,
            background,
            camera,
            box_outlines,
            effect_passes,
            mut buffer_pool,
            chunks_render_bundle,
        ): Self::SystemData,
    ) {
        use specs::Join;

        let mut texture = render_state.surface.get_current_texture().unwrap();

        let frame = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let bloom_texture = buffer_pool.get_buffer();

        let bloom_frame = bloom_texture.create_view(&TextureViewDescriptor::default());

        let normal_map_frame = effect_passes
            .normal_map
            .create_view(&TextureViewDescriptor::default());
        let position_map_frame = effect_passes
            .position_map
            .create_view(&TextureViewDescriptor::default());

        let view_projection_bind_group = render_state.projection_bind_group.take().unwrap();
        let view_projection_fragment_bind_group =
            render_state.fragment_projection_bind_group.take().unwrap();

        let frame_image_buffer = buffer_pool.get_buffer();

        let frame_image_view = frame_image_buffer.create_view(&TextureViewDescriptor::default());

        let mut encoder = get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Main Render Loop Command Encoder"),
        });

        {
            if game_state.state == ProgramState::InGame {
                background.draw(
                    &frame_image_view,
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
                        wgpu::RenderPassColorAttachment {
                            view: &normal_map_frame,
                            resolve_target: None,
                            ops: Operations {
                                load: LoadOp::Clear(Color::BLACK),
                                store: true,
                            },
                        },
                        wgpu::RenderPassColorAttachment {
                            view: &position_map_frame,
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

                if let Some(bundle) = &chunks_render_bundle.bundle {
                    render_pass.execute_bundles(bundle.iter());
                }

                mem::drop(render_pass);

                effect_passes.apply_bloom(
                    &mut encoder,
                    &mut buffer_pool,
                    &*bloom_texture,
                    &frame_image_buffer,
                );

                effect_passes.apply_ssao(
                    &mut encoder,
                    &mut buffer_pool,
                    &view_projection_fragment_bind_group,
                    &*frame_image_buffer,
                );

                // Merge vfx buffer into main swapchain
                encoder.copy_texture_to_texture(
                    frame_image_buffer.as_image_copy(),
                    texture.texture.as_image_copy(),
                    get_swapchain_size(),
                );

                render_state.outlines.render(
                    &frame,
                    &mut encoder,
                    &view_projection_bind_group,
                    &box_outlines,
                    &render_state.depth_texture.1,
                );
            }

            ui_service.render(
                &texture.texture,
                &mut encoder,
                &asset_service,
                &mut render_state.queue,
            );

            // Return buffers
            buffer_pool.return_buffer(frame_image_buffer);
            buffer_pool.return_buffer(bloom_texture);

            // Clean used buffers
            buffer_pool.clean_buffers(&mut encoder);

            render_state.queue.submit(Some(encoder.finish()));

            texture.present();
        }

        std::mem::drop(frame);

        render_state.projection_bind_group = Some(view_projection_bind_group);
        render_state.fragment_projection_bind_group = Some(view_projection_fragment_bind_group);
    }
}
