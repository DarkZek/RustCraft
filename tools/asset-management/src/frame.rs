use imgui::{MouseCursor};
use std::time::Instant;
use crate::screens::model_selection::ModelSelection;
use crate::render::render::RenderState;

pub struct DataStore {
    model_selection: ModelSelection
}

impl DataStore {

    pub fn new() -> DataStore {
        DataStore {
            model_selection: ModelSelection::new()
        }
    }

    pub fn draw_frame(&mut self, last_frame: &mut Instant, render: &mut RenderState, last_cursor: &mut Option<MouseCursor>) {

        *last_frame = render.imgui.io_mut().update_delta_time(*last_frame);

        let frame = match render.swap_chain.get_next_texture() {
            Ok(frame) => frame,
            Err(e) => {
                eprintln!("dropped frame: {:?}", e);
                return;
            }
        };

        render.platform
            .prepare_frame(render.imgui.io_mut(), &render.window)
            .expect("Failed to prepare frame");
        let ui = render.imgui.frame();

        {
            self.model_selection.frame(&ui);
            ui.show_demo_window(&mut true);
        }

        let mut encoder: wgpu::CommandEncoder =
            render.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        if last_cursor != &mut ui.mouse_cursor() {
            *last_cursor = ui.mouse_cursor();
            render.platform.prepare_render(&ui, &render.window);
        }
        render.renderer
            .render(ui.render(), &mut render.device, &mut encoder, &frame.view)
            .expect("Rendering failed");

        render.queue.submit(&[encoder.finish()]);
    }
}