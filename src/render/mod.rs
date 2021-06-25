use crate::render::camera::Camera;
use crate::render::loading::LoadingScreen;
use crate::render::pass::uniforms::Uniforms;
use crate::render::shaders::load_shaders;
use crate::services::asset_service::depth_map::{create_depth_texture, DEPTH_FORMAT};
use crate::services::asset_service::AssetService;
use crate::services::chunk_service::mesh::Vertex;
use crate::services::chunk_service::ChunkService;
use crate::services::{load_services, ServicesContext};
use image::ImageFormat;
use specs::{World, WorldExt};
use std::borrow::Borrow;
use std::lazy::SyncOnceCell;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use wgpu::{
    AdapterInfo, BindGroupLayout, BlendComponent, DepthBiasState, Device, Face, FrontFace,
    MultisampleState, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, Sampler,
    StencilState, SwapChainDescriptor, Texture, TextureFormat, TextureView, VertexState,
};
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Icon, Window, WindowBuilder};

pub mod camera;
pub mod device;
pub mod loading;
pub mod pass;
pub mod screens;
pub mod shaders;

lazy_static! {
    pub static ref TEXTURE_FORMAT: SyncOnceCell<TextureFormat> = SyncOnceCell::new();
}

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

    pub fps: u32,
    pub frames: u32,
    frame_capture_time: Instant,

    gpu_info: AdapterInfo,

    last_frame_time: SystemTime,
    delta_time: Duration,
}

impl RenderState {
    pub fn new(universe: &mut World, event_loop: &EventLoop<()>) -> Self {
        let last_frame_time = SystemTime::now();
        let delta_time = Duration::from_millis(0);

        let icon_img = image::load_from_memory_with_format(
            include_bytes!("../../../RustCraft/assets/logo.png"),
            ImageFormat::Png,
        )
        .unwrap();

        let icon = Icon::from_rgba(icon_img.to_bytes(), 128, 128).unwrap();

        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Loading - Rustcraft")
                .with_inner_size(PhysicalSize {
                    width: 1536,
                    height: 864,
                })
                .with_window_icon(Some(icon))
                .build(&event_loop)
                .unwrap(),
        );

        // Get the window setup ASAP so we can show loading screen
        let (size, surface, gpu_info, device, queue, adapter) =
            RenderState::get_devices(window.borrow());

        log!(format!("Using {:?} {} {} with backend {:?}", gpu_info.device_type, gpu_info.name, gpu_info.vendor, gpu_info.backend));

        // Convert to forms that can be used in multiple places
        let device = Arc::new(device);
        let queue = Arc::new(Mutex::new(queue));

        // Limited to vsync here
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        TEXTURE_FORMAT
            .set(sc_desc.format)
            .expect("Failed to update texture format description");

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        let swap_chain = Arc::new(Mutex::new(swap_chain));

        // Start showing loading screen
        let loading = LoadingScreen::new(&size, swap_chain.clone(), device.clone(), queue.clone());
        loading.start_loop();

        // Start the intensive job of loading services
        load_services(
            ServicesContext::new(device.clone(), queue.clone(), &size, window.clone()),
            universe,
        );

        LoadingScreen::update_state(95.0);

        // TODO: Combine uniforms into camera
        let mut camera = Camera::new(&size);
        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&mut camera);
        let (uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
            uniforms.create_uniform_buffers(&device.clone());

        universe.insert(camera);

        let depth_texture = create_depth_texture(&device.clone(), &sc_desc);

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
                &universe
                    .read_resource::<ChunkService>()
                    .model_bind_group_layout,
            ],
        );

        // Send the notice to shut the loading screen down
        LoadingScreen::update_state(100.0);

        // Loop until it has
        loop {
            if LoadingScreen::has_finished() {
                break;
            }
            thread::sleep(Duration::from_millis(25));
        }

        // Spin lock until the background loading thread shuts down, and has dropped the swapchain
        let swap_chain = LoadingScreen::wait_for_swapchain(swap_chain);

        let queue = Arc::try_unwrap(queue).ok().unwrap().into_inner().unwrap();

        let device = Arc::try_unwrap(device).ok().unwrap();

        window.set_title("RustCraft");

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
            fps: 0,
            frames: 0,
            frame_capture_time: Instant::now(),
            gpu_info,
            last_frame_time,
            delta_time,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;

        // Current swapchain must be dropped before creating a new one
        {
            self.swap_chain.take();
        }

        self.swap_chain = Some(self.device.create_swap_chain(&self.surface, &self.sc_desc));
        self.depth_texture = create_depth_texture(&self.device, &self.sc_desc);
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
    let (vs_module, fs_module) = load_shaders(
        device,
        (
            include_bytes!("../../../RustCraft/assets/shaders/shader_vert.spv"),
            include_bytes!("../../../RustCraft/assets/shaders/shader_frag.spv"),
        ),
    );

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Main render pipeline layout"),
        bind_group_layouts,
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Main render pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: VertexState {
            module: &vs_module,
            entry_point: "main",
            buffers: &[Vertex::desc()],
        },
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Cw,
            cull_mode: Some(Face::Back),
            clamp_depth: false,
            polygon_mode: PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: StencilState {
                front: wgpu::StencilFaceState::IGNORE,
                back: wgpu::StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
            bias: DepthBiasState::default(),
        }),
        multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(wgpu::FragmentState {
            module: &fs_module,
            entry_point: "main",
            targets: &[wgpu::ColorTargetState {
                format: TEXTURE_FORMAT.get().unwrap().clone(),
                write_mask: wgpu::ColorWrite::ALL,
                blend: Some(wgpu::BlendState {
                    color: BlendComponent {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: BlendComponent {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::Zero,
                        operation: wgpu::BlendOperation::Add,
                    },
                }),
            }],
        }),
    })
}
