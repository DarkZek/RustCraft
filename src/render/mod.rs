use winit::window::Window;
use crate::render::camera::Camera;
use crate::render::pass::uniforms::Uniforms;
use wgpu::{Texture, TextureView, Sampler, AdapterInfo, RenderPipeline, Device, BindGroupLayout, SwapChainDescriptor};
use crate::block::{blocks, Block};
use crate::render::shaders::load_shaders;
use std::time::Instant;
use systemstat::{System, Platform};
use crate::services::asset_service::depth_map::{create_depth_texture, DEPTH_FORMAT};
use crate::services::{Services, ServicesContext};
use crate::render::loading::LoadingScreen;
use crate::services::chunk_service::mesh::Vertex;

pub mod pass;
pub mod camera;
pub mod shaders;
pub mod screens;
pub mod device;
pub mod loading;

pub struct RenderState {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: Option<wgpu::SwapChain>,

    render_pipeline: wgpu::RenderPipeline,

    size: winit::dpi::PhysicalSize<u32>,

    pub camera: Camera,
    pub uniforms: Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    depth_texture: (Texture, TextureView, Sampler),

    blocks: Vec<Block>,

    fps: u32,
    fps_counter: Instant,
    frames: u32,

    gpu_info: AdapterInfo,
    system_info: System,

    pub(crate) services: Option<Services>
}

impl RenderState {
    pub fn new(window: &Window) -> Self {

        // Get the window setup ASAP so we can show loading screen
        let (size, surface, gpu_info, mut device, mut queue) = RenderState::get_devices(&window);

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Vsync,
        };

        let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

        // Start showing loading screen
        let mut loading = LoadingScreen::new(&device, &size);
        loading.render(&mut swap_chain, &device, &mut queue, 10);

        let mut blocks = blocks::get_blocks();

        // Start the intensive job of loading services
        let mut services = Services::load_services(ServicesContext::new(&mut device, &mut queue, &mut blocks, &size));

        //Change to 50 %
        loading.render(&mut swap_chain, &device, &mut queue, 90);

        // TODO: Combine uniforms into camera
        let camera = Camera::new(&size);
        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera);
        let (uniform_buffer, uniform_bind_group_layout, uniform_bind_group) = uniforms.create_uniform_buffers(&device);

        // Hand the atlas to the renderer
        //let (atlas_sampler, atlas_texture, atlas_mapping) = (services.asset.texture_sampler.take().unwrap(), services.asset.texture_atlas.take().unwrap(), services.asset.texture_atlas_index.take().unwrap());

        let depth_texture = create_depth_texture(&device, &sc_desc);

        let render_pipeline = generate_render_pipeline(&sc_desc, &device,
                                                       &[&services.asset.atlas_bind_group_layout.as_ref().unwrap(), &uniform_bind_group_layout, &services.chunk.bind_group_layout]);

        //Load font

        let system_info = System::new();

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain: Some(swap_chain),
            size,
            render_pipeline,
            camera,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            depth_texture,
            blocks,
            fps: 0,
            fps_counter: Instant::now(),
            frames: 0,
            gpu_info,
            system_info,
            services: Some(services)
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = Some(self.device.create_swap_chain(&self.surface, &self.sc_desc));
        self.depth_texture = create_depth_texture(&self.device, &self.sc_desc);
        self.camera.aspect = new_size.width as f32 / new_size.height as f32;

        let services = self.services.take().unwrap();
        services.ui.update_ui_projection_matrix(self, new_size);
        self.services = Some(services);
    }
}

fn generate_render_pipeline(sc_desc: &SwapChainDescriptor, device: &Device, bind_group_layouts: &[&BindGroupLayout]) -> RenderPipeline{

    let (vs_module, fs_module) = load_shaders(device);

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &[
            Vertex::desc(),
        ],
        layout: &render_pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fs_module,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Cw,
            cull_mode: wgpu::CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        color_states: &[
            wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            },
        ],
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
            format: DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
            stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
            stencil_read_mask: 0,
            stencil_write_mask: 0,
        }),
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false
    })
}