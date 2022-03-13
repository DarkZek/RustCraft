use crate::render::vertices::Vertex;
use nalgebra::Vector3;
use rc_ui::atlas::Rotate;
use wgpu::Buffer;

pub mod chunk;
pub mod culling;
pub mod generation;
pub mod rerendering;
pub mod update;

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

    pub fn invert(&self) -> ViewableDirectionBitMap {
        match self {
            ViewableDirectionBitMap::Top => ViewableDirectionBitMap::Bottom,
            ViewableDirectionBitMap::Bottom => ViewableDirectionBitMap::Top,
            ViewableDirectionBitMap::Left => ViewableDirectionBitMap::Right,
            ViewableDirectionBitMap::Right => ViewableDirectionBitMap::Left,
            ViewableDirectionBitMap::Front => ViewableDirectionBitMap::Back,
            ViewableDirectionBitMap::Back => ViewableDirectionBitMap::Front,
        }
    }

    pub fn rotate(&self, deg: Rotate) -> ViewableDirectionBitMap {
        match deg {
            // Rotate::Deg270 => match self {
            //     ViewableDirectionBitMap::Top => ViewableDirectionBitMap::Top,
            //     ViewableDirectionBitMap::Bottom => ViewableDirectionBitMap::Top,
            //     ViewableDirectionBitMap::Left => ViewableDirectionBitMap::Front,
            //     ViewableDirectionBitMap::Right => ViewableDirectionBitMap::Back,
            //     ViewableDirectionBitMap::Front => ViewableDirectionBitMap::Left,
            //     ViewableDirectionBitMap::Back => ViewableDirectionBitMap::Right,
            // },
            Rotate::Deg180 => self.clone(),
            Rotate::Deg90 | Rotate::Deg270 => match self {
                ViewableDirectionBitMap::Top => ViewableDirectionBitMap::Bottom,
                ViewableDirectionBitMap::Bottom => ViewableDirectionBitMap::Top,
                ViewableDirectionBitMap::Left => ViewableDirectionBitMap::Back,
                ViewableDirectionBitMap::Right => ViewableDirectionBitMap::Front,
                ViewableDirectionBitMap::Front => ViewableDirectionBitMap::Right,
                ViewableDirectionBitMap::Back => ViewableDirectionBitMap::Left,
            },
            _ => {
                log_warn!("Rotate not implemented");
                *self
            }
        }
    }
}
