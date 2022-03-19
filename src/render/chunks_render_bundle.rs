use crate::render::device::get_device;
use crate::render::{get_texture_format, RenderState};
use crate::services::asset_service::depth_map::DEPTH_FORMAT;
use crate::services::asset_service::AssetService;
use crate::services::chunk_service::chunk::{ChunkData, Chunks};
use crate::services::chunk_service::ChunkService;
use specs::{Join, ReadStorage};
use wgpu::{
    IndexFormat, RenderBundle, RenderBundleDepthStencil, RenderBundleDescriptor,
    RenderBundleEncoderDescriptor, TextureFormat,
};

pub struct ChunksRenderBundle {
    pub bundle: Option<[RenderBundle; 1]>,
}

impl Default for ChunksRenderBundle {
    fn default() -> Self {
        todo!()
    }
}

impl ChunksRenderBundle {
    pub fn new() -> ChunksRenderBundle {
        ChunksRenderBundle { bundle: None }
    }

    pub fn create_render_bundle(
        &mut self,
        render_state: &RenderState,
        asset_service: &AssetService,
        chunk_service: &ChunkService,
        chunks: &ReadStorage<ChunkData>,
    ) {
        let chunks = Chunks::new(chunks.join().collect::<Vec<&ChunkData>>());

        let mut encoder =
            get_device().create_render_bundle_encoder(&RenderBundleEncoderDescriptor {
                label: Some("Chunks Render Bundle Encoder"),
                color_formats: &[
                    *get_texture_format(),
                    *get_texture_format(),
                    TextureFormat::Rgba16Float,
                    TextureFormat::Rgba16Float,
                ],
                depth_stencil: Some(RenderBundleDepthStencil {
                    format: DEPTH_FORMAT,
                    depth_read_only: false,
                    stencil_read_only: false,
                }),
                sample_count: 1,
                multiview: None,
            });

        encoder.set_pipeline(&render_state.render_pipeline);

        encoder.set_bind_group(0, &asset_service.atlas_bind_group.as_ref().unwrap(), &[]);
        encoder.set_bind_group(1, render_state.projection_bind_group.as_ref().unwrap(), &[]);

        for pos in &chunk_service.visible_chunks {
            let chunk = chunks.get_loc(*pos).unwrap();
            if chunk.opaque_model.indices_buffer.is_none() {
                continue;
            }
            let indices_buffer = chunk.opaque_model.indices_buffer.as_ref().unwrap();
            let vertices_buffer = chunk.opaque_model.vertices_buffer.as_ref().unwrap();
            let model_bind_group = chunk.model_bind_group.as_ref().unwrap();

            encoder.set_bind_group(2, model_bind_group, &[]);
            encoder.set_vertex_buffer(0, vertices_buffer.slice(..));
            encoder.set_index_buffer(indices_buffer.slice(..), IndexFormat::Uint16);
            encoder.draw_indexed(0..chunk.opaque_model.indices_buffer_len, 0, 0..1);
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

            encoder.set_bind_group(2, model_bind_group, &[]);
            encoder.set_vertex_buffer(0, vertices_buffer.slice(..));
            encoder.set_index_buffer(indices_buffer.slice(..), IndexFormat::Uint16);
            encoder.draw_indexed(0..chunk.translucent_model.indices_buffer_len, 0, 0..1);
        }

        let render_buffer = encoder.finish(&RenderBundleDescriptor {
            label: Some("Chunks Render Bundle"),
        });

        self.bundle = Some([render_buffer]);
    }
}
