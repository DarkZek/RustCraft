use crate::services::ui_service::UIService;
use wgpu::{CommandEncoder, Device, SwapChainOutput};
use crate::services::asset_service::AssetService;

impl UIService {

    /// Renders the user interface. This also runs all of the sub managers render functions.
    pub fn render(&self,
                  frame: &SwapChainOutput,
                  encoder: &mut CommandEncoder,
                  device: &Device,
                  asset_service: &AssetService
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Load,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
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

        render_pass.set_vertex_buffer(0, &vertices, 0, 0);
        render_pass.set_index_buffer(indices, 0, 0);
        render_pass.draw_indexed(0..indices_len, 0, 0..1);
    }
}
