use crate::render::device::get_device;
use crate::render::loading::models::load_splash;
use crate::render::vertices::UIVertex;
use lazy_static::lazy_static;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupLayout, Buffer, BufferUsages, Queue, RenderPipeline, Sampler,
    Surface, Texture,
};
use winit::dpi::PhysicalSize;

pub mod models;
pub mod pass;
pub mod render;

lazy_static! {
    pub static ref LOADING_STATE: Mutex<f32> = Mutex::new(0.0);
}

// Vertices that show the static loading screen elements
const STANDARD_VERTICES: [UIVertex; 18] = [
    UIVertex {
        position: [0.72, -0.49],
        tex_coords: [-1.0, -1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    UIVertex {
        position: [-0.72, -0.49],
        tex_coords: [-1.0, -1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    UIVertex {
        position: [0.72, -0.61],
        tex_coords: [-1.0, -1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    UIVertex {
        position: [-0.72, -0.61],
        tex_coords: [-1.0, -1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    UIVertex {
        position: [0.72, -0.61],
        tex_coords: [-1.0, -1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    UIVertex {
        position: [-0.72, -0.49],
        tex_coords: [-1.0, -1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    UIVertex {
        position: [0.71, -0.5],
        tex_coords: [-1.0, -1.0],
        color: [0.937, 0.196, 0.239, 1.0],
    },
    UIVertex {
        position: [-0.71, -0.5],
        tex_coords: [-1.0, -1.0],
        color: [0.937, 0.196, 0.239, 1.0],
    },
    UIVertex {
        position: [0.71, -0.6],
        tex_coords: [-1.0, -1.0],
        color: [0.937, 0.196, 0.239, 1.0],
    },
    UIVertex {
        position: [-0.71, -0.6],
        tex_coords: [-1.0, -1.0],
        color: [0.937, 0.196, 0.239, 1.0],
    },
    UIVertex {
        position: [0.71, -0.6],
        tex_coords: [-1.0, -1.0],
        color: [0.937, 0.196, 0.239, 1.0],
    },
    UIVertex {
        position: [-0.71, -0.5],
        tex_coords: [-1.0, -1.0],
        color: [0.937, 0.196, 0.239, 1.0],
    },
    UIVertex {
        position: [0.7, 0.2],
        tex_coords: [1.0, 0.0],
        color: [1.0, 1.0, 1.0, 0.0],
    },
    UIVertex {
        position: [-0.7, 0.2],
        tex_coords: [0.0, 0.0],
        color: [1.0, 1.0, 1.0, 0.0],
    },
    UIVertex {
        position: [0.7, -0.2],
        tex_coords: [1.0, 1.0],
        color: [1.0, 1.0, 1.0, 0.0],
    },
    UIVertex {
        position: [-0.7, -0.2],
        tex_coords: [0.0, 1.0],
        color: [0.0, 0.0, 0.0, 0.0],
    },
    UIVertex {
        position: [0.7, -0.2],
        tex_coords: [1.0, 1.0],
        color: [0.0, 0.0, 0.0, 0.0],
    },
    UIVertex {
        position: [-0.7, 0.2],
        tex_coords: [0.0, 0.0],
        color: [0.0, 0.0, 0.0, 0.0],
    },
];

///
/// This is a self contained render pipeline responsible for showing the loading screen before other assets are loaded.
/// This is separate so it can be ran before the other services are setup.
/// It needs to use a lot of Arcs and Mutex's so it can draw to the screen while things that also require them can still be setup.
///
pub struct LoadingScreen {
    pipeline: RenderPipeline,
    queue: Arc<Mutex<Queue>>,
    surface: Arc<Surface>,
    default_vertices_buffer: Buffer,
    splash_texture: Texture,
    splash_sampler: Sampler,
    splash_bind_group_layout: BindGroupLayout,
    splash_bind_group: BindGroup,
    view_buffer: Buffer,
    view_bindgroup: BindGroup,
    view_bindgroup_layout: BindGroupLayout,
}

impl LoadingScreen {
    pub fn new(
        size: &PhysicalSize<u32>,
        surface: Arc<Surface>,
        queue: Arc<Mutex<Queue>>,
    ) -> LoadingScreen {
        let default_vertices_buffer = load_buffers();

        let (splash_texture, splash_sampler, splash_bind_group_layout, splash_bind_group) =
            load_splash(queue.lock().unwrap().deref_mut());

        let (view_buffer, view_bindgroup, view_bindgroup_layout) =
            LoadingScreen::setup_ui_projection_matrix(size.clone());

        let pipeline = LoadingScreen::generate_loading_render_pipeline(&[
            &splash_bind_group_layout,
            &view_bindgroup_layout,
        ]);

        LoadingScreen {
            pipeline,
            surface,
            queue,
            default_vertices_buffer,
            splash_texture,
            splash_sampler,
            splash_bind_group_layout,
            splash_bind_group,
            view_buffer,
            view_bindgroup,
            view_bindgroup_layout,
        }
    }

    pub fn update_state(state: f32) {
        *crate::render::loading::LOADING_STATE.lock().unwrap() = state;
    }

    pub fn has_finished() -> bool {
        *crate::render::loading::LOADING_STATE.lock().unwrap() == -1.0
    }

    // pub fn wait_for_swapchain(mut swap_chain: Arc<Mutex<SwapChain>>) -> SwapChain {
    //     loop {
    //         // Try to unwrap
    //         let chain = Arc::try_unwrap(swap_chain);
    //
    //         // Decide output
    //         match chain {
    //             Ok(out) => {
    //                 // If success unwrap mutex and return SwapChain
    //                 return out.into_inner().unwrap();
    //             }
    //             Err(swap_chain_out) => {
    //                 // If error, return variable and loop again
    //                 swap_chain = swap_chain_out;
    //             }
    //         }
    //
    //         thread::sleep(Duration::from_millis(1000 / 60));
    //     }
    // }
}

pub fn load_buffers() -> Buffer {
    let defaults_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
        label: Some("Loading Vertices Buffer"),
        contents: &bytemuck::cast_slice(STANDARD_VERTICES.as_ref()),
        usage: BufferUsages::VERTEX,
    });

    defaults_buffer
}
