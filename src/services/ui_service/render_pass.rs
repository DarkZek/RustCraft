use crate::services::ui_service::UIService;
use wgpu::{SwapChainOutput, CommandEncoder, Device};
use crate::services::Services;

impl UIService {
    pub fn render(frame: &SwapChainOutput, encoder: &mut CommandEncoder, device: &Device, services: &mut Services) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[
                wgpu::RenderPassColorAttachmentDescriptor {
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
                }
            ],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&services.ui.pipeline);
        render_pass.set_bind_group(0, &services.asset.atlas_bind_group.as_ref().unwrap(), &[]);
        render_pass.set_bind_group(1, &services.ui.projection_bind_group, &[]);

        // Draw fonts
        services.ui.fonts.total(device);
        let vertices = services.ui.fonts.total_vertex_buffer.as_ref().unwrap();
        let indices_len = services.ui.fonts.total_indices.len() as u32;
        let indices = services.ui.fonts.total_indices_buffer.as_ref().unwrap();

        render_pass.set_vertex_buffers(0, &[(vertices, 0)]);
        render_pass.set_index_buffer(indices, 0);
        render_pass.draw_indexed(0..indices_len, 0, 0..1);
    }
}