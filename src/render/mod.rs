use crate::render::background::Background;
use crate::render::camera::Camera;
use crate::render::device::get_device;
use crate::render::effects::EffectPasses;
use crate::render::loading::LoadingScreen;
use crate::render::pass::outline::{BoxOutline, OutlineRenderer};
use crate::render::pass::uniforms::RenderViewProjectionUniforms;
use crate::render::vertices::Vertex;
use crate::services::asset_service::depth_map::{create_depth_texture, DEPTH_FORMAT};
use crate::services::asset_service::AssetService;
use crate::services::chunk_service::ChunkService;
use crate::services::settings_service::SettingsService;
use crate::services::{load_services, ServicesContext};
use crate::world::player_selected_block_update::PlayerSelectedBlockUpdateSystemData;
use image::ImageFormat;
use nalgebra::Vector3;
use rc_ui::vertex::UIVertex;
use specs::{Builder, World, WorldExt};
use std::borrow::Borrow;
use std::lazy::SyncOnceCell;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    AdapterInfo, BindGroupLayout, BlendComponent, BufferUsages, DepthBiasState, Extent3d, Face,
    FrontFace, MultisampleState, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline,
    Sampler, StencilState, Texture, TextureFormat, TextureView, VertexState,
};
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Icon, Window, WindowBuilder};

pub mod background;
pub mod camera;
pub mod device;
pub mod effects;
pub mod loading;
pub mod pass;
pub mod screens;
pub mod vertices;

#[rustfmt::skip]
lazy_static! {
    pub static ref TEXTURE_FORMAT: SyncOnceCell<TextureFormat> = SyncOnceCell::new();

    // Buffers that holds vertices that will cover the entire screen when drawn with no view matrix
    // Texture goes from 0,0 to 1,1
    pub static ref VERTICES_COVER_SCREEN: SyncOnceCell<wgpu::Buffer> = SyncOnceCell::new();
}

pub static mut SWAPCHAIN_SIZE: Extent3d = Extent3d {
    width: 0,
    height: 0,
    depth_or_array_layers: 1,
};

pub fn get_texture_format() -> &'static TextureFormat {
    TEXTURE_FORMAT.get().unwrap()
}

pub fn get_swapchain_size() -> Extent3d {
    unsafe { SWAPCHAIN_SIZE }
}

/// Stores the current state of the rendering side of the game. This includes all of the vulkan attributes, fps, gpu info and currently the services which need to be moved.
#[allow(dead_code)]
pub struct RenderState {
    surface: Arc<wgpu::Surface>,
    surface_desc: wgpu::SurfaceConfiguration,

    pub queue: wgpu::Queue,
    pub window: Arc<Window>,

    render_pipeline: wgpu::RenderPipeline,

    size: PhysicalSize<u32>,

    pub uniform_buffer: wgpu::Buffer,
    projection_bind_group: Option<wgpu::BindGroup>,
    fragment_projection_bind_group: Option<wgpu::BindGroup>,

    depth_texture: (Texture, TextureView, Sampler),

    outlines: OutlineRenderer,

    pub fps: u32,
    pub frames: u32,
    frame_capture_time: Instant,

    pub gpu_info: AdapterInfo,

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
                    width: 1280,
                    height: 720,
                })
                .with_window_icon(Some(icon))
                //.with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
                .build(&event_loop)
                .unwrap(),
        );

        // Get the window setup ASAP so we can show loading screen
        let (size, surface, gpu_info, queue, adapter) = RenderState::get_devices(window.borrow());

        log!(format!(
            "Using {:?} {} {} with backend {:?}",
            gpu_info.device_type, gpu_info.name, gpu_info.vendor, gpu_info.backend
        ));

        // Convert to forms that can be used in multiple places
        let queue = Arc::new(Mutex::new(queue));

        // Limited to vsync here
        let surface_desc = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_DST,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };

        unsafe {
            SWAPCHAIN_SIZE = Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            }
        }

        surface.configure(get_device(), &surface_desc);

        TEXTURE_FORMAT
            .set(surface_desc.format)
            .expect("Failed to update texture format description");

        let surface = Arc::new(surface);

        // Start showing loading screen
        let loading = LoadingScreen::new(&size, surface.clone(), queue.clone());
        loading.start_loop();

        universe.insert(Background::new());

        // Start the intensive job of loading services
        load_services(
            ServicesContext::new(queue.clone(), &size, window.clone()),
            universe,
        );

        LoadingScreen::update_state(95.0);

        // TODO: Combine uniforms into camera
        let camera = Camera::new(&size);

        let (
            uniform_buffer,
            uniform_bind_group_layout,
            uniform_bind_group,
            _fragment_bind_group_layout,
            fragment_bind_group,
        ) = RenderViewProjectionUniforms.create_uniform_buffers();

        universe.insert(camera);

        fill_vertices_cover_screen();

        // TODO: Find better place
        let mut box_outline = BoxOutline::new(
            Vector3::new(-1.0, 69.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
            [0.0; 4],
        );
        box_outline.build();
        universe.create_entity().with(box_outline).build();

        let t = PlayerSelectedBlockUpdateSystemData::new(universe);
        universe.insert(t);

        let depth_texture = create_depth_texture(&surface_desc);

        let render_pipeline = generate_render_pipeline(
            universe.read_resource::<SettingsService>().backface_culling,
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

        // Create effects buffers
        let effects = EffectPasses::new(
            &mut queue.lock().unwrap(),
            &universe.read_resource::<SettingsService>(),
        );

        universe.insert(effects);

        let queue = Arc::try_unwrap(queue).ok().unwrap().into_inner().unwrap();

        let outlines = OutlineRenderer::new();

        window.set_title("RustCraft");

        Self {
            surface,
            surface_desc,
            queue,
            window,
            size,
            render_pipeline,
            uniform_buffer,
            projection_bind_group: Some(uniform_bind_group),
            fragment_projection_bind_group: Some(fragment_bind_group),
            depth_texture,
            outlines,
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
        self.surface_desc.width = new_size.width;
        self.surface_desc.height = new_size.height;

        unsafe {
            SWAPCHAIN_SIZE = Extent3d {
                width: new_size.width,
                height: new_size.height,
                depth_or_array_layers: 1,
            };
        }

        self.depth_texture = create_depth_texture(&self.surface_desc);
        self.surface.configure(get_device(), &self.surface_desc);
    }
}

impl Default for RenderState {
    fn default() -> Self {
        unimplemented!()
    }
}

fn generate_render_pipeline(
    culling: bool,
    bind_group_layouts: &[&BindGroupLayout],
) -> RenderPipeline {
    let vs_module =
        get_device().create_shader_module(&wgpu::include_spirv!("../../shaders/shader_vert.spv"));
    let fs_module =
        get_device().create_shader_module(&wgpu::include_spirv!("../../shaders/shader_frag.spv"));

    let render_pipeline_layout =
        get_device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Main render pipeline layout"),
            bind_group_layouts,
            push_constant_ranges: &[],
        });

    get_device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
            cull_mode: if culling { Some(Face::Back) } else { None },
            unclipped_depth: false,
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
            targets: &[
                // Color output
                wgpu::ColorTargetState {
                    format: *get_texture_format(),
                    write_mask: wgpu::ColorWrites::ALL,
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
                },
                // Bloom pass brightness threshold
                wgpu::ColorTargetState {
                    format: *get_texture_format(),
                    write_mask: wgpu::ColorWrites::ALL,
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
                },
                // Normal map
                wgpu::ColorTargetState {
                    format: TextureFormat::Rgba16Float,
                    write_mask: wgpu::ColorWrites::ALL,
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
                },
                // Position map
                wgpu::ColorTargetState {
                    format: TextureFormat::Rgba16Float,
                    write_mask: wgpu::ColorWrites::ALL,
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
                },
            ],
        }),
        multiview: None,
    })
}

fn fill_vertices_cover_screen() {
    let vertex_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
        label: Some("Cover Screen Vertices Buffer"),
        contents: &bytemuck::cast_slice(&[
            UIVertex {
                position: [-1.0, 1.0],
                tex_coords: [0.0, 0.0],
                color: [0.0; 4],
            },
            UIVertex {
                position: [1.0, 1.0],
                tex_coords: [1.0, 0.0],
                color: [0.0; 4],
            },
            UIVertex {
                position: [-1.0, -1.0],
                tex_coords: [0.0, 1.0],
                color: [0.0; 4],
            },
            UIVertex {
                position: [1.0, 1.0],
                tex_coords: [1.0, 0.0],
                color: [0.0; 4],
            },
            UIVertex {
                position: [1.0, -1.0],
                tex_coords: [1.0, 1.0],
                color: [0.0; 4],
            },
            UIVertex {
                position: [-1.0, -1.0],
                tex_coords: [0.0, 1.0],
                color: [0.0; 4],
            },
        ]),
        usage: BufferUsages::VERTEX,
    });

    VERTICES_COVER_SCREEN.set(vertex_buffer).unwrap();
}
