use crate::block::{blocks, Block};
use crate::render::camera::Camera;
use crate::render::loading::LoadingScreen;
use crate::render::pass::uniforms::Uniforms;
use crate::render::shaders::load_shaders;
use crate::services::asset_service::depth_map::{create_depth_texture, DEPTH_FORMAT};
use crate::services::asset_service::AssetService;
use crate::services::chunk_service::mesh::Vertex;
use crate::services::chunk_service::ChunkService;
use crate::services::{load_services, ServicesContext};
use specs::{World, WorldExt};
use std::borrow::Borrow;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use systemstat::{Platform, System};
use wgpu::{
    AdapterInfo, BindGroupLayout, Device, RenderPipeline, Sampler, SwapChainDescriptor, Texture,
    TextureView, VertexStateDescriptor,
};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub mod camera;
pub mod device;
pub mod loading;
pub mod pass;
pub mod screens;
pub mod shaders;

/// Stores the current state of the rendering side of the game. This includes all of the vulkan attributes, fps, gpu info and currently the services which need to be moved.
#[allow(dead_code)]
pub struct RenderState {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub window: Arc<Window>,

    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: Option<wgpu::SwapChain>,

    render_pipeline: wgpu::RenderPipeline,

    size: winit::dpi::PhysicalSize<u32>,

    pub uniforms: Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    depth_texture: (Texture, TextureView, Sampler),

    blocks: Vec<Block>,

    pub fps: u32,
    pub frames: u32,
    frame_capture_time: Instant,

    gpu_info: AdapterInfo,
    system_info: System,

    last_frame_time: SystemTime,
    delta_time: Duration,
}

impl RenderState {
    pub fn new(universe: &mut World, event_loop: &EventLoop<()>) -> Self {
        let last_frame_time = SystemTime::now();
        let delta_time = Duration::from_millis(0);

        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Loading - Rustcraft")
                .build(&event_loop)
                .unwrap(),
        );

        // Get the window setup ASAP so we can show loading screen
        let (size, surface, gpu_info, mut device, mut queue) =
            RenderState::get_devices(window.borrow());

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

        // Start showing loading screen
        let mut loading = LoadingScreen::new(&device, &size);
        loading.render(&mut swap_chain, &device, &mut queue, 10);

        let mut blocks = blocks::get_blocks();

        // Start the intensive job of loading services
        load_services(
            ServicesContext::new(&mut device, &mut queue, &mut blocks, &size, window.clone()),
            universe,
        );

        // Update loading screen
        loading.render(&mut swap_chain, &device, &mut queue, 90);

        // TODO: Combine uniforms into camera
        let camera = Camera::new(&size);
        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera);
        let (uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
            uniforms.create_uniform_buffers(&device);

        universe
            .write_resource::<ChunkService>()
            .update_frustum_culling(&camera);
        universe.insert(camera);

        // Hand the atlas to the renderer
        //let (atlas_sampler, atlas_texture, atlas_mapping) = (services.asset.texture_sampler.take().unwrap(), services.asset.texture_atlas.take().unwrap(), services.asset.texture_atlas_index.take().unwrap());

        let depth_texture = create_depth_texture(&device, &sc_desc);

        let render_pipeline = generate_render_pipeline(
            &sc_desc,
            &device,
            true,
            &[
                &universe
                    .read_resource::<AssetService>()
                    .atlas_bind_group_layout
                    .as_ref()
                    .unwrap(),
                &uniform_bind_group_layout,
                &universe.read_resource::<ChunkService>().bind_group_layout,
            ],
        );

        let system_info = System::new();

        Self {
            surface,
            device,
            queue,
            window,
            sc_desc,
            swap_chain: Some(swap_chain),
            size,
            render_pipeline,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            depth_texture,
            blocks,
            fps: 0,
            frames: 0,
            frame_capture_time: Instant::now(),
            gpu_info,
            system_info,
            last_frame_time,
            delta_time,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = Some(self.device.create_swap_chain(&self.surface, &self.sc_desc));
        self.depth_texture = create_depth_texture(&self.device, &self.sc_desc);

        // let mut services = self.services.take().unwrap();
        // services.ui.update_ui_projection_matrix(self, new_size);
        // self.services = Some(services);
    }
}

impl Default for RenderState {
    fn default() -> Self {
        unimplemented!()
    }
}

fn generate_render_pipeline(
    sc_desc: &SwapChainDescriptor,
    device: &Device,
    culling: bool,
    bind_group_layouts: &[&BindGroupLayout],
) -> RenderPipeline {
    let (vs_module, fs_module) = load_shaders(device);

    let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { bind_group_layouts });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
            cull_mode: if culling {
                wgpu::CullMode::Back
            } else {
                wgpu::CullMode::None
            },
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        color_states: &[wgpu::ColorStateDescriptor {
            format: sc_desc.format,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
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
        vertex_state: VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[Vertex::desc()],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    })
}
