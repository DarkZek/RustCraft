use crate::services::ui_service::UIService;
use wgpu::{CommandEncoder, Device, Operations, LoadOp, SwapChainFrame};
use crate::services::asset_service::AssetService;

impl UIService {

    /// Renders the user interface. This also runs all of the sub managers render functions.
    pub fn render(&self,
                  frame: &SwapChainFrame,
                  encoder: &mut CommandEncoder,
                  device: &Device,
                  asset_service: &AssetService
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.output.view,
                resolve_target: None,
                ops: Operations { load: LoadOp::Load, store: true }
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &asset_service.atlas_bind_group.as_ref().unwrap(), &[]);
        render_pass.set_bind_group(1, &self.projection_bind_group, &[]);

        // Draw fonts
        let vertices = self.fonts.total_vertex_buffer.as_ref().unwrap();
        let indices_len = self.fonts.total_indices.len() as u32;
        let indices = self.fonts.total_indices_buffer.as_ref().unwrap();

        render_pass.set_vertex_buffer(0, vertices.slice(..));
        render_pass.set_index_buffer(indices.slice(..));
        render_pass.draw_indexed(0..indices_len, 0, 0..1);
    }
}
