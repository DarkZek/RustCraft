use wgpu::{SwapChain, RenderPipeline, Device, Queue, BufferUsage, Buffer, Sampler, BindGroupLayout, BindGroup, Texture};
use std::sync::{Arc, Mutex};
use winit::dpi::PhysicalSize;
use image::ImageFormat;
use instant::Duration;
use std::thread;
use lazy_static::lazy_static;
use crate::services::chunk_service::mesh::UIVertex;
use crate::render::loading::models::load_splash;
use std::ops::DerefMut;

pub mod render;
pub mod pass;
pub mod models;

lazy_static! {
    pub static ref LOADING_STATE: Mutex<f32> = Mutex::new(0.0);
}

const STANDARD_VERTICES: [UIVertex; 18] = [
    UIVertex { position: [0.73, -0.49], tex_coords: [-1.0, -1.0], color: [1.0, 1.0, 1.0, 1.0] },
    UIVertex { position: [-0.73, -0.49], tex_coords: [-1.0, -1.0], color: [1.0, 1.0, 1.0, 1.0] },
    UIVertex { position: [0.73, -0.61], tex_coords: [-1.0, -1.0], color: [1.0, 1.0, 1.0, 1.0] },

    UIVertex { position: [-0.73, -0.61], tex_coords: [-1.0, -1.0], color: [1.0, 1.0, 1.0, 1.0] },
    UIVertex { position: [0.73, -0.61], tex_coords: [-1.0, -1.0], color: [1.0, 1.0, 1.0, 1.0] },
    UIVertex { position: [-0.73, -0.49], tex_coords: [-1.0, -1.0], color: [1.0, 1.0, 1.0, 1.0] },

    UIVertex { position: [0.72, -0.5], tex_coords: [-1.0, -1.0], color: [0.937, 0.196, 0.239, 1.0] },
    UIVertex { position: [-0.72, -0.5], tex_coords: [-1.0, -1.0], color: [0.937, 0.196, 0.239, 1.0] },
    UIVertex { position: [0.72, -0.6], tex_coords: [-1.0, -1.0], color: [0.937, 0.196, 0.239, 1.0] },

    UIVertex { position: [-0.72, -0.6], tex_coords: [-1.0, -1.0], color: [0.937, 0.196, 0.239, 1.0] },
    UIVertex { position: [0.72, -0.6], tex_coords: [-1.0, -1.0], color: [0.937, 0.196, 0.239, 1.0] },
    UIVertex { position: [-0.72, -0.5], tex_coords: [-1.0, -1.0], color: [0.937, 0.196, 0.239, 1.0] },

    UIVertex { position: [0.74, 0.2], tex_coords: [1.0, 0.0], color: [1.0, 1.0, 1.0, 0.0] },
    UIVertex { position: [-0.74, 0.2], tex_coords: [0.0, 0.0], color: [1.0, 1.0, 1.0, 0.0] },
    UIVertex { position: [0.74, -0.2], tex_coords: [1.0, 1.0], color: [1.0, 1.0, 1.0, 0.0] },

    UIVertex { position: [-0.74, -0.2], tex_coords: [0.0, 1.0], color: [0.0, 0.0, 0.0, 0.0] },
    UIVertex { position: [0.74, -0.2], tex_coords: [1.0, 1.0], color: [0.0, 0.0, 0.0, 0.0] },
    UIVertex { position: [-0.74, 0.2], tex_coords: [0.0, 0.0], color: [0.0, 0.0, 0.0, 0.0] },
];

///
/// This is a self contained render pipeline responsible for showing the loading screen before other assets are loaded.
/// This is separate so it can be ran before the other services are setup.
/// It needs to use a lot of Arcs and Mutex's so it can draw to the screen while things that also require them can still be setup.
///
pub struct LoadingScreen {
    pipeline: RenderPipeline,
    swapchain: Arc<Mutex<SwapChain>>,
    device: Arc<Device>,
    queue: Arc<Mutex<Queue>>,
    default_vertices_buffer: Buffer,
    splash_texture: Texture,
    splash_sampler: Sampler,
    splash_bind_group_layout: BindGroupLayout,
    splash_bind_group: BindGroup,
    view_buffer: Buffer,
    view_bindgroup: BindGroup,
    view_bindgroup_layout: BindGroupLayout
}

impl LoadingScreen {
    pub fn new(size: &PhysicalSize<u32>,
               swapchain: Arc<Mutex<SwapChain>>,
               device: Arc<Device>,
               queue: Arc<Mutex<Queue>>) -> LoadingScreen {

        let default_vertices_buffer = load_buffers(device.as_ref());

        let (splash_texture, splash_sampler, splash_bind_group_layout, splash_bind_group)
            = load_splash(device.as_ref(), queue.lock().unwrap().deref_mut());

        let (view_buffer, view_bindgroup, view_bindgroup_layout) = LoadingScreen::setup_ui_projection_matrix(size.clone(), device.as_ref());

        let pipeline = LoadingScreen::generate_loading_render_pipeline(device.as_ref(), &[&splash_bind_group_layout, &view_bindgroup_layout]);

        LoadingScreen {
            pipeline,
            swapchain,
            device,
            queue,
            default_vertices_buffer,
            splash_texture,
            splash_sampler,
            splash_bind_group_layout,
            splash_bind_group,
            view_buffer,
            view_bindgroup,
            view_bindgroup_layout
        }
    }

    pub fn update_state(state: f32) {
        *crate::render::loading::LOADING_STATE
            .lock()
            .unwrap() = state;
    }

    pub fn wait_for_swapchain(mut swap_chain: Arc<Mutex<SwapChain>>) -> SwapChain {
        loop {
            // Try to unwrap
            let chain = Arc::try_unwrap(swap_chain);

            // Decide output
            match chain {
                Ok(out) => {
                    // If success unwrap mutex and return SwapChain
                    return out.into_inner().unwrap();
                },
                Err(swap_chain_out) => {
                    // If error, return variable and loop again
                    swap_chain = swap_chain_out;
                },
            }

            thread::sleep(Duration::from_millis(1000 / 60));
        }
    }
}

pub fn load_buffers(device: &Device) -> Buffer {
    let defaults_buffer = device.create_buffer_with_data(
        bytemuck::cast_slice(STANDARD_VERTICES.as_ref()),
        BufferUsage::VERTEX,
    );

    defaults_buffer
}