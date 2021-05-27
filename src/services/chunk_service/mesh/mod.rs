use nalgebra::Vector3;
use wgpu::Buffer;

pub mod chunk;
pub mod culling;
pub mod generation;
pub mod rerendering;
pub mod update;

//TODO: Maybe look into using some shader trickery to decrease VRAM usage by generating indices shader side since it never changes

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normals: [f32; 3],
    pub applied_color: [u8; 4],
}

#[derive(Debug)]
pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub vertices_buffer: Option<Buffer>,
    pub indices_buffer: Option<Buffer>,
    pub indices_buffer_len: u32,
}

impl MeshData {
    pub fn default() -> Self {
        MeshData {
            vertices: vec![],
            indices: vec![],
            vertices_buffer: None,
            indices_buffer: None,
            indices_buffer_len: 0,
        }
    }
}

unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UIVertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}

unsafe impl bytemuck::Zeroable for UIVertex {}
unsafe impl bytemuck::Pod for UIVertex {}

pub struct WGPU4x4Matrix {
    pub x: [f32; 4],
    pub y: [f32; 4],
    pub z: [f32; 4],
    pub w: [f32; 4],
}

impl WGPU4x4Matrix {}

#[derive(Clone, Copy, PartialEq)]
pub enum ViewableDirectionBitMap {
    Top = 0b00000001,
    Bottom = 0b00000010,
    Left = 0b00000100,
    Right = 0b00001000,
    Front = 0b00010000,
    Back = 0b00100000,
}

impl ViewableDirectionBitMap {
    pub fn from(direction: &Vector3<i32>) -> ViewableDirectionBitMap {
        if direction.z > 0 {
            ViewableDirectionBitMap::Back
        } else if direction.z < 0 {
            ViewableDirectionBitMap::Front
        } else if direction.y > 0 {
            ViewableDirectionBitMap::Top
        } else if direction.y < 0 {
            ViewableDirectionBitMap::Bottom
        } else if direction.x < 0 {
            ViewableDirectionBitMap::Left
        } else if direction.x > 0 {
            ViewableDirectionBitMap::Right
        } else {
            ViewableDirectionBitMap::Top
        }
    }
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() + mem::size_of::<[f32; 2]>())
                        as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>()
                        + mem::size_of::<[f32; 2]>()
                        + mem::size_of::<[f32; 3]>())
                        as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Unorm8x4,
                },
            ],
        }
    }
}

impl UIVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<UIVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (2 * mem::size_of::<[f32; 2]>()) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}
