use crate::services::asset_service::AssetService;
use crate::services::ui_service::UIService;
use wgpu::{
    CommandEncoder, IndexFormat, LoadOp, Operations, Queue, Texture, TextureView,
    TextureViewDescriptor,
};

impl UIService {
    /// Renders the user interface. This also runs all of the sub managers render functions.
    pub fn render(
        &mut self,
        frame: &Texture,
        encoder: &mut CommandEncoder,
        asset_service: &AssetService,
        queue: &mut Queue,
    ) {
        self.controller.process(queue);
        self.controller.render(frame, encoder);

        let frame = frame.create_view(&TextureViewDescriptor::default());

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("UI Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &frame,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &asset_service.atlas_bind_group.as_ref().unwrap(), &[]);
        render_pass.set_bind_group(1, &self.projection_bind_group, &[]);

        if self.images.model.total_indices.len() != 0 {
            // Draw images
            let vertices = self.images.model.total_vertex_buffer.as_ref().unwrap();
            let indices_len = self.images.model.total_indices.len() as u32;
            let indices = self.images.model.total_indices_buffer.as_ref().unwrap();

            render_pass.set_vertex_buffer(0, vertices.slice(..));
            render_pass.set_index_buffer(indices.slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..indices_len, 0, 0..1);
        }

        if self.widget.model.total_indices.len() != 0 {
            // Draw widgets
            let vertices = self.widget.model.total_vertex_buffer.as_ref().unwrap();
            let indices_len = self.widget.model.total_indices.len() as u32;
            let indices = self.widget.model.total_indices_buffer.as_ref().unwrap();

            render_pass.set_vertex_buffer(0, vertices.slice(..));
            render_pass.set_index_buffer(indices.slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..indices_len, 0, 0..1);
        }

        if self.fonts.model.total_indices.len() != 0 {
            // Draw fonts
            let vertices = self.fonts.model.total_vertex_buffer.as_ref().unwrap();
            let indices_len = self.fonts.model.total_indices.len() as u32;
            let indices = self.fonts.model.total_indices_buffer.as_ref().unwrap();

            render_pass.set_vertex_buffer(0, vertices.slice(..));
            render_pass.set_index_buffer(indices.slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..indices_len, 0, 0..1);
        }
    }
}
