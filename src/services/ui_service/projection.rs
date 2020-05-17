use cgmath::{Ortho, Matrix4};
use wgpu::{Buffer, BindGroup, BindGroupLayout};
use crate::services::ServicesContext;
use crate::services::ui_service::UIService;
use winit::dpi::PhysicalSize;
use crate::render::RenderState;
use crate::render::camera::OPENGL_TO_WGPU_MATRIX;

impl UIService {
    pub fn setup_ui_projection_matrix(context: &mut ServicesContext) -> (Buffer, BindGroup, BindGroupLayout) {

        log!("Setting up screen with size: {}", format!("{}x{}", context.size.width, context.size.height));

        let projection = Ortho {
            left: -(context.size.width as f32 / 2.0),
            right: context.size.width as f32 / 2.0,
            bottom: context.size.height as f32 / 2.0,
            top: -(context.size.height as f32 / 2.0),
            near: 0.1,
            far: 10.0
        };

        let matrix_binding_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false
                    },
                }
            ]
        };

        let matrix: Matrix4<f32> = projection.into();

        log!("Orthographic Matrix: {:?}", matrix);

        let matrix_buffer = context.device
            .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&[matrix]);

        let matrix_bind_group_layout = context.device.create_bind_group_layout(&matrix_binding_layout_descriptor);

        let matrix_bind_group_descriptor = wgpu::BindGroupDescriptor {
            layout: &matrix_bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &matrix_buffer,
                        range: 0..std::mem::size_of_val(&matrix) as wgpu::BufferAddress,
                    }
                }
            ],
        };

        let matrix_bind_group = context.device.create_bind_group(&matrix_bind_group_descriptor);

        (matrix_buffer, matrix_bind_group, matrix_bind_group_layout)
    }

    pub fn update_ui_projection_matrix(&mut self, render: &mut RenderState, size: PhysicalSize<u32>) {
        let projection = Ortho {
            left: -(size.width as f32 / 2.0),
            right: size.width as f32 / 2.0,
            bottom: size.height as f32 / 2.0,
            top: -(size.height as f32 / 2.0),
            near: 0.1,
            far: 10.0
        };

        let mut matrix: Matrix4<f32> = projection.into();
        //TODO: Remove this when WGPU switches its axis
        matrix = matrix * OPENGL_TO_WGPU_MATRIX;
        let mut encoder = render.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        let matrix_buffer = render.device
            .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&[matrix]);

        self.fonts.resized(&size, &render.device);

        encoder.copy_buffer_to_buffer(&matrix_buffer, 0x0, &self.projection_buffer, 0x0, std::mem::size_of_val(&matrix) as wgpu::BufferAddress);

        render.queue.submit(&[
            encoder.finish()
        ]);
    }
}