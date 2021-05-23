use crate::helpers::Lerp;
use crate::render::loading::{LoadingScreen, STANDARD_VERTICES};
use crate::services::chunk_service::mesh::UIVertex;
use instant::Instant;
use std::thread;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BufferUsage, Color, LoadOp, Operations};

impl LoadingScreen {
    pub fn start_loop(mut self) {
        thread::spawn(move || {
            let mut displayed_progress: f32 = 0.0;
            let mut target_progress = 0.0;

            let mut update_progress = 0;

            let mut processed_fps = 60;
            let mut fps = 0;
            let mut time = Instant::now();

            loop {
                displayed_progress =
                    displayed_progress.lerp(target_progress, 2.0 / processed_fps as f32);

                // Only update progress every 15 frames to reduce lag
                if update_progress == 15 {
                    target_progress = *crate::render::loading::LOADING_STATE.lock().unwrap();
                    update_progress = 0;
                } else {
                    update_progress += 1;
                }

                if target_progress >= 100.0 {
                    self.render(100.0);
                    break;
                }
                fps += 1;

                if time.elapsed().as_secs_f32() > 1.0 {
                    time = Instant::now();
                    processed_fps = fps;
                    fps = 0;
                }

                self.render(displayed_progress);
            }

            // Signal we've shut down the thread and dropped the swapchain
            *crate::render::loading::LOADING_STATE.lock().unwrap() = -1.0;
        });
    }

    pub fn render(&mut self, percentage: f32) {
        let x = ((percentage as f32 / 100.0) * 1.2) - 0.6;

        let top_left = UIVertex {
            position: [-0.7, -0.51],
            tex_coords: [-1.0, -1.0],
            color: [1.0, 1.0, 1.0, 1.0],
        };
        let top_right = UIVertex {
            position: [x, -0.51],
            tex_coords: [-1.0, -1.0],
            color: [1.0, 1.0, 1.0, 1.0],
        };
        let bottom_left = UIVertex {
            position: [-0.7, -0.59],
            tex_coords: [-1.0, -1.0],
            color: [1.0, 1.0, 1.0, 1.0],
        };
        let bottom_right = UIVertex {
            position: [x, -0.59],
            tex_coords: [-1.0, -1.0],
            color: [1.0, 1.0, 1.0, 1.0],
        };

        // Create loading
        let vertices = vec![
            top_left,
            bottom_right,
            bottom_left,
            top_left,
            top_right,
            bottom_right,
        ];

        let vertices_buffer = self
            .device
            .as_ref()
            .create_buffer_init(&BufferInitDescriptor {
                label: Some("Loading verticles buffer descriptor"),
                contents: &bytemuck::cast_slice(vertices.as_slice()),
                usage: BufferUsage::VERTEX,
            });

        let frame = self.swapchain.lock().unwrap().get_current_frame().unwrap();

        let mut encoder =
            self.device
                .as_ref()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Loading command encoder descriptor"),
                });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Loading Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.output.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            //r: 239.0 / 255.0,
                            r: 2.0,
                            g: 50.0 / 255.0,
                            b: 61.0 / 255.0,
                            a: 0.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.splash_bind_group, &[]);
            render_pass.set_bind_group(1, &self.view_bindgroup, &[]);

            render_pass.set_vertex_buffer(0, self.default_vertices_buffer.slice(..));
            render_pass.draw(0..STANDARD_VERTICES.len() as u32, 0..1);

            render_pass.set_vertex_buffer(0, vertices_buffer.slice(..));
            render_pass.draw(0..vertices.len() as u32, 0..1);
        }

        self.queue.lock().unwrap().submit(Some(encoder.finish()));
    }
}
