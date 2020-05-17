use cgmath::Vector3;

pub mod chunk;
pub mod culling;
pub mod block;
pub mod debug;
pub mod generation;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    /*pub relative_coords: [f32; 2],*/
    pub normals: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UIVertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4]
}

pub enum ViewableDirectionBitMap {
    Top     = 0b00000001,
    Bottom  = 0b00000010,
    Left    = 0b00000100,
    Right   = 0b00001000,
    Front   = 0b00010000,
    Back    = 0b00100000,
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
    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: (mem::size_of::<[f32; 3]>() + mem::size_of::<[f32; 2]>()) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float3,
                },
            ]
        }
    }
}

impl UIVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<UIVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: (2 * mem::size_of::<[f32; 2]>()) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float4,
                },
            ]
        }
    }
}