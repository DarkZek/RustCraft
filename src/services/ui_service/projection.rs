use wgpu::{Buffer, BindGroup, BindGroupLayout};
use crate::services::ServicesContext;
use crate::services::ui_service::UIService;
use winit::dpi::PhysicalSize;
use crate::render::RenderState;
use nalgebra::{Orthographic3, Matrix4};

impl UIService {
    pub fn setup_ui_projection_matrix(context: &mut ServicesContext) -> (Buffer, BindGroup, BindGroupLayout) {

        log!("Setting up screen with size: {}", format!("{}x{}", context.size.width, context.size.height));

        let projection = Orthographic3::new(
            -(context.size.width as f32 / 2.0),
            context.size.width as f32 / 2.0,
            context.size.height as f32 / 2.0,
            -(context.size.height as f32 / 2.0),
            0.1,
            10.0
        );

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

        let projection = Orthographic3::new(
            -(size.width as f32 / 2.0),
            size.width as f32 / 2.0,
            size.height as f32 / 2.0,
            -(size.height as f32 / 2.0),
            0.1,
            10.0
        );

        let opengl_to_wgpu_matrix: Matrix4<f32> = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, -1.0, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.0,
            0.0, 0.0, 0.5, 1.0,
        );

        let mut matrix: Matrix4<f32> = projection.into();
        //TODO: Remove this when WGPU switches its axis
        matrix = matrix * opengl_to_wgpu_matrix;
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