use winit::window::Window;
use crate::render::mesh::generation::generate_terrain;
use crate::render::mesh::Vertex;
use crate::render::camera::Camera;
use crate::render::pass::uniforms::Uniforms;
use crate::render::texture::depth_map::{create_depth_texture, DEPTH_FORMAT};
use wgpu::{Texture, TextureView, Sampler};
use crate::block::{blocks, Block};
use crate::render::shaders::load_shaders;
use crate::render::texture::atlas::TextureAtlasIndex;

pub mod mesh;
pub mod pass;
pub mod texture;
pub mod camera;
pub mod shaders;

pub struct RenderState {
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

    render_pipeline: wgpu::RenderPipeline,

    vertex_buffer: wgpu::Buffer,

    indices_buffer: wgpu::Buffer,
    indices_buffer_len: u32,

    size: winit::dpi::PhysicalSize<u32>,

    diffuse_texture: wgpu::Texture,
    diffuse_texture_view: wgpu::TextureView,
    diffuse_sampler: wgpu::Sampler,
    diffuse_bind_group: wgpu::BindGroup,

    pub camera: Camera,
    pub uniforms: Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    depth_texture: (Texture, TextureView, Sampler),

    atlas_mapping: Vec<TextureAtlasIndex>,
    blocks: Vec<Block>
}

impl RenderState {
    pub fn new(window: &Window) -> Self {

        let size = window.inner_size();

        let surface = wgpu::Surface::create(window);

        let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
            ..Default::default()
        }).unwrap();

        let (device, mut queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: Default::default(),
        });

        let mut blocks = blocks::get_blocks();

        let (sampler, atlas_texture, atlas_mapping) = texture::mapping::load_textures(&mut blocks, &mut queue, &device);

        let (texture_bind_group_layout, diffuse_bind_group, diffuse_texture_view) = texture::binding::binding_layout(&device, &atlas_texture, &sampler);

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Vsync,
        };

        let camera = Camera::new(&size);

        let mut uniforms = Uniforms::new();

        uniforms.update_view_proj(&camera);

        let (uniform_buffer, uniform_bind_group_layout, uniform_bind_group) = uniforms.create_uniform_buffers(&device);

        let (mut terrain, mut tindices) = generate_terrain(&blocks);
        let vertices = terrain.as_mut_slice();
        let indices = tindices.as_mut_slice();

        let vertex_buffer = device
            .create_buffer_mapped(vertices.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(vertices);

        let indices_buffer = device
            .create_buffer_mapped(indices.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(indices);

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let (vs_module, fs_module) = load_shaders(&device);

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&texture_bind_group_layout, &uniform_bind_group_layout],
        });

        let depth_texture = create_depth_texture(&device, &sc_desc);

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
            alpha_to_coverage_enabled: false,
        });

        Self {
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            render_pipeline,
            vertex_buffer,
            indices_buffer,
            indices_buffer_len: tindices.len() as u32,
            diffuse_texture: atlas_texture,
            diffuse_texture_view,
            diffuse_sampler: sampler,
            diffuse_bind_group,
            camera,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            depth_texture,
            atlas_mapping,
            blocks
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        self.depth_texture = create_depth_texture(&self.device, &self.sc_desc);
        self.camera.aspect = new_size.width as f32 / new_size.height as f32;
    }
}