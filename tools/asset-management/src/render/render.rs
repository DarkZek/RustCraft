use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::dpi::{LogicalSize, PhysicalSize};
use futures::executor::block_on;
use imgui::{FontSource, Context};
use imgui_wgpu::Renderer;
use wgpu::{SwapChain, Queue, Device, SwapChainDescriptor, Surface};
use imgui_winit_support::WinitPlatform;

pub struct RenderState {
    pub window: Window,
    pub window_size: PhysicalSize<u32>,
    pub device: Device,
    pub queue: Queue,
    pub swap_chain: SwapChain,
    pub imgui: Context,
    pub renderer: Renderer,
    pub hidpi_factor: f64,
    pub sc_desc: SwapChainDescriptor,
    pub surface: Surface,
    pub platform: WinitPlatform
}

impl RenderState {
    pub fn new(event_loop: &EventLoop<()>) -> RenderState {

        let hidpi_factor = 1.0;
        let (window, window_size, surface) = {
            let version = env!("CARGO_PKG_VERSION");

            let window = Window::new(&event_loop).unwrap();
            window.set_inner_size(LogicalSize {
                width: 1280.0,
                height: 720.0,
            });
            window.set_title(&format!("Rustcraft Asset Management {}", version));
            let size = window.inner_size();

            let surface = wgpu::Surface::create(&window);

            (window, size, surface)
        };

        let adapter = block_on(wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        )).unwrap();

        let (device, mut queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        }));


        // Set up swap chain
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: window_size.width as u32,
            height: window_size.height as u32,
            present_mode: wgpu::PresentMode::Mailbox,
        };


        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        // Set up dear imgui
        let mut imgui = imgui::Context::create();
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            &window,
            imgui_winit_support::HiDpiMode::Default,
        );
        imgui.set_ini_filename(None);

        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        //
        // Set up dear imgui wgpu renderer
        //
        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 0.0,
        };

        #[cfg(not(feature = "glsl-to-spirv"))]
            let renderer = Renderer::new(
            &mut imgui,
            &device,
            &mut queue,
            sc_desc.format,
            Some(clear_color),
        );

        #[cfg(feature = "glsl-to-spirv")]
        let mut renderer = Renderer::new_glsl(
            &mut imgui,
            &device,
            &mut queue,
            sc_desc.format,
            Some(clear_color),
        );

        RenderState {
            window,
            window_size,
            device,
            queue,
            swap_chain,
            imgui,
            renderer,
            hidpi_factor,
            sc_desc,
            surface,
            platform
        }
    }
}